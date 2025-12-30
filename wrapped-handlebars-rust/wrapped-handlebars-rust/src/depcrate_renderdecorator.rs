// Generated macro for Decorator (struct)
macro_rules! Depcrate_renderDecorator {
() => {
// Module: crate::render
// Provides: {"Decorator"}
// Dependencies: {}
# [doc = " Render-time Decorator data when using in a decorator definition"] # [derive (Debug)] pub struct Decorator < 'rc > { name : Cow < 'rc , str > , params : Vec < PathAndJson < 'rc > > , hash : BTreeMap < & 'rc str , PathAndJson < 'rc > > , template : Option < & 'rc Template > , indent : Option < Cow < 'rc , str > > , }
};
}
