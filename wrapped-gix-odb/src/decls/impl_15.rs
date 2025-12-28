macro_rules! deps {
    () => {
        Error!();
        Store!();
        Handle!();
        Header!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < S > crate :: Header for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { fn try_header (& self , id : & oid) -> Result < Option < Header > , gix_object :: find :: Error > { let mut snapshot = self . snapshot . borrow_mut () ; let mut inflate = self . inflate . borrow_mut () ; self . try_header_inner (id , & mut inflate , & mut snapshot , None) . map_err (| err | Box :: new (err) as _) } }
    };
}

impl_15!();