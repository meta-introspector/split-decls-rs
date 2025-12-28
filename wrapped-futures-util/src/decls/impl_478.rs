macro_rules! deps {
    () => {
        Ready!();
        Pending!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < St1 , St2 > Stream for Zip < St1 , St2 > where St1 : Stream , St2 : Stream , { type Item = (St1 :: Item , St2 :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if this . queued1 . is_none () { match this . stream1 . as_mut () . poll_next (cx) { Poll :: Ready (Some (item1)) => * this . queued1 = Some (item1) , Poll :: Ready (None) | Poll :: Pending => { } } } if this . queued2 . is_none () { match this . stream2 . as_mut () . poll_next (cx) { Poll :: Ready (Some (item2)) => * this . queued2 = Some (item2) , Poll :: Ready (None) | Poll :: Pending => { } } } if this . queued1 . is_some () && this . queued2 . is_some () { let pair = (this . queued1 . take () . unwrap () , this . queued2 . take () . unwrap ()) ; Poll :: Ready (Some (pair)) } else if this . stream1 . is_done () || this . stream2 . is_done () { Poll :: Ready (None) } else { Poll :: Pending } } fn size_hint (& self) -> (usize , Option < usize >) { let queued1_len = usize :: from (self . queued1 . is_some ()) ; let queued2_len = usize :: from (self . queued2 . is_some ()) ; let (stream1_lower , stream1_upper) = self . stream1 . size_hint () ; let (stream2_lower , stream2_upper) = self . stream2 . size_hint () ; let stream1_lower = stream1_lower . saturating_add (queued1_len) ; let stream2_lower = stream2_lower . saturating_add (queued2_len) ; let lower = cmp :: min (stream1_lower , stream2_lower) ; let upper = match (stream1_upper , stream2_upper) { (Some (x) , Some (y)) => { let x = x . saturating_add (queued1_len) ; let y = y . saturating_add (queued2_len) ; Some (cmp :: min (x , y)) } (Some (x) , None) => x . checked_add (queued1_len) , (None , Some (y)) => y . checked_add (queued2_len) , (None , None) => None , } ; (lower , upper) } }
    };
}

impl_478!();