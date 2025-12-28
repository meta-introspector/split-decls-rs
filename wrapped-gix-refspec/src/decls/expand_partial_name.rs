macro_rules! expand_partial_name {
    () => {
        pub (crate) fn expand_partial_name < T > (name : & BStr , mut cb : impl FnMut (& BStr) -> Option < T >) -> Option < T > { use bstr :: ByteVec ; let mut buf = BString :: from (Vec :: with_capacity (128)) ; for (base , append_head) in [("" , false) , ("refs/" , false) , ("refs/tags/" , false) , ("refs/heads/" , false) , ("refs/remotes/" , false) , ("refs/remotes/" , true) ,] { buf . clear () ; buf . push_str (base) ; buf . push_str (name) ; if append_head { buf . push_str ("/HEAD") ; } if let Some (res) = cb (buf . as_ref ()) { return Some (res) ; } } None }
    };
}

expand_partial_name!()