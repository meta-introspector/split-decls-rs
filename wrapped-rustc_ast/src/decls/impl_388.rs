macro_rules! impl_388 {
    () => {
        impl NodeId { pub fn placeholder_from_expn_id (expn_id : LocalExpnId) -> Self { NodeId :: from_u32 (expn_id . as_u32 ()) } pub fn placeholder_to_expn_id (self) -> LocalExpnId { LocalExpnId :: from_u32 (self . as_u32 ()) } }
    };
}

impl_388!();