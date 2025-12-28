macro_rules! deps {
    () => {
        Handle!();
        Store!();
        Header!();
        Error!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < S > gix_object :: FindHeader for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < gix_object :: Header > , gix_object :: find :: Error > { let mut snapshot = self . snapshot . borrow_mut () ; let mut inflate = self . inflate . borrow_mut () ; self . try_header_inner (id , & mut inflate , & mut snapshot , None) . map (| maybe_header | { maybe_header . map (| hdr | gix_object :: Header { kind : hdr . kind () , size : hdr . size () , }) }) . map_err (| err | Box :: new (err) as _) } }
    };
}

impl_7!()