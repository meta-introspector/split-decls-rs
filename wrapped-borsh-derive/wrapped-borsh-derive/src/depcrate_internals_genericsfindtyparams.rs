// Generated macro for FindTyParams (struct)
macro_rules! Depcrate_internals_genericsFindTyParams {
() => {
// Module: crate::internals::generics
// Provides: {"FindTyParams"}
// Dependencies: {}
# [doc = " a Visitor-like struct, which helps determine, if a type parameter is found in field"] # [derive (Clone)] pub struct FindTyParams { all_type_params : HashSet < Ident > , all_type_params_ordered : Vec < Ident > , relevant_type_params : HashSet < Ident > , associated_type_params_usage : HashMap < Ident , Vec < Type > > , }
};
}
