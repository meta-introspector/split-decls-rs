macro_rules! deps {
    () => {
        UnindexedConsumer!();
        Iter!();
        UnindexedRangeLen!();
        UnindexedProducer!();
        Folder!();
        IterProducer!();
    };
}

macro_rules! unindexed_range_impl {
    () => {
        deps!();
        macro_rules ! unindexed_range_impl { ($ t : ty , $ len_t : ty) => { impl UnindexedRangeLen <$ len_t > for Range <$ t > { fn unindexed_len (& self) -> $ len_t { let & Range { start , end } = self ; if end > start { end . wrapping_sub (start) as $ len_t } else { 0 } } } impl RangeInteger for $ t { private_impl ! { } fn drive_unindexed < C > (iter : Iter <$ t >, consumer : C) -> C :: Result where C : UnindexedConsumer <$ t >, { # [inline] fn offset (start : $ t) -> impl Fn (usize) -> $ t { move | i | start . wrapping_add (i as $ t) } if let Some (len) = iter . opt_len () { (0 .. len) . into_par_iter () . map (offset (iter . range . start)) . drive (consumer) } else { bridge_unindexed (IterProducer { range : iter . range } , consumer) } } fn opt_len (iter : & Iter <$ t >) -> Option < usize > { usize :: try_from (iter . range . unindexed_len ()) . ok () } } impl UnindexedProducer for IterProducer <$ t > { type Item = $ t ; fn split (mut self) -> (Self , Option < Self >) { let index = self . range . unindexed_len () / 2 ; if index > 0 { let mid = self . range . start . wrapping_add (index as $ t) ; let right = mid .. self . range . end ; self . range . end = mid ; (self , Some (IterProducer { range : right })) } else { (self , None) } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item >, { folder . consume_iter (self) } } } ; }
    };
}

unindexed_range_impl!()