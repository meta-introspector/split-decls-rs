// Generated macro for impl_640 (impl)
macro_rules! Depcrate_options_outer_fromimpl_640 {
() => {
// Module: crate::options::outer_from
// Provides: {"impl_640"}
// Dependencies: {}
impl OuterFrom { pub fn start (di : & syn :: DeriveInput) -> Result < Self > { Ok (OuterFrom { container : Core :: start (di) ? , attrs : Default :: default () , ident : Default :: default () , attr_names : Default :: default () , forward_attrs : Default :: default () , from_ident : Default :: default () , }) } pub fn as_forward_attrs (& self) -> ForwardAttrs < '_ > { ForwardAttrs { field : self . attrs . as_ref () , filter : self . forward_attrs . as_ref () , } } }
};
}
