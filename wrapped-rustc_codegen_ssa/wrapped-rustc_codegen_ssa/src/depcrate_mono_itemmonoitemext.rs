// Generated macro for MonoItemExt (trait)
macro_rules! Depcrate_mono_itemMonoItemExt {
() => {
// Module: crate::mono_item
// Provides: {"MonoItemExt"}
// Dependencies: {}
pub trait MonoItemExt < 'a , 'tcx > { fn define < Bx : BuilderMethods < 'a , 'tcx > > (& self , cx : & 'a mut Bx :: CodegenCx , cgu_name : & str , item_data : MonoItemData ,) ; fn predefine < Bx : BuilderMethods < 'a , 'tcx > > (& self , cx : & 'a mut Bx :: CodegenCx , cgu_name : & str , linkage : Linkage , visibility : Visibility ,) ; fn to_raw_string (& self) -> String ; }
};
}
