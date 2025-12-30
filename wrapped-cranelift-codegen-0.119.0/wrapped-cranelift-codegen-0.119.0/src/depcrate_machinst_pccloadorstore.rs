// Generated macro for LoadOrStore (enum)
macro_rules! Depcrate_machinst_pccLoadOrStore {
() => {
// Module: crate::machinst::pcc
// Provides: {"LoadOrStore"}
// Dependencies: {}
# [doc = " The operation we're checking against an amode: either"] # [doc = ""] # [doc = " - a *load*, and we need to validate that the field's fact subsumes"] # [doc = "   the load result's fact, OR"] # [doc = ""] # [doc = " - a *store*, and we need to validate that the stored data's fact"] # [doc = "   subsumes the field's fact."] pub (crate) enum LoadOrStore < 'a > { Load { result_fact : Option < & 'a Fact > , from_bits : u16 , to_bits : u16 , } , Store { stored_fact : Option < & 'a Fact > , } , }
};
}
