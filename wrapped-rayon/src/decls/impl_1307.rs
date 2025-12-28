macro_rules! deps {
    () => {
        UnindexedProducer!();
        Folder!();
        EncodeUtf16Producer!();
    };
}

macro_rules! impl_1307 {
    () => {
        deps!();
        impl < 'ch > UnindexedProducer for EncodeUtf16Producer < 'ch > { type Item = u16 ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (EncodeUtf16Producer { chars : left } , Some (EncodeUtf16Producer { chars : right }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume_iter (self . chars . encode_utf16 ()) } }
    };
}

impl_1307!();