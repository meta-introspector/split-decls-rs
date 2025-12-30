// Generated macro for Helper (struct)
macro_rules! Depcrate_renderHelper {
() => {
// Module: crate::render
// Provides: {"Helper"}
// Dependencies: {}
# [doc = " Render-time Helper data when using in a helper definition"] # [derive (Debug , Clone)] pub struct Helper < 'rc > { name : Cow < 'rc , str > , params : Vec < PathAndJson < 'rc > > , hash : BTreeMap < & 'rc str , PathAndJson < 'rc > > , template : Option < & 'rc Template > , inverse : Option < & 'rc Template > , block_param : Option < & 'rc BlockParam > , block : bool , }
};
}
