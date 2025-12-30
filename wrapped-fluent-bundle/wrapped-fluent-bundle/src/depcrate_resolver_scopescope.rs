// Generated macro for Scope (struct)
macro_rules! Depcrate_resolver_scopeScope {
() => {
// Module: crate::resolver::scope
// Provides: {"Scope"}
// Dependencies: {}
# [doc = " State for a single `ResolveValue::to_value` call."] pub struct Scope < 'bundle , 'ast , 'args , 'errors , R , M > { # [doc = " The current `FluentBundle` instance."] pub bundle : & 'bundle FluentBundle < R , M > , # [doc = " The current arguments passed by the developer."] pub (super) args : Option < & 'args FluentArgs < 'args > > , # [doc = " Local args"] pub (super) local_args : Option < FluentArgs < 'bundle > > , # [doc = " The running count of resolved placeables. Used to detect the Billion"] # [doc = " Laughs and Quadratic Blowup attacks."] pub (super) placeables : u8 , # [doc = " Tracks hashes to prevent infinite recursion."] traveled : smallvec :: SmallVec < & 'ast ast :: Pattern < & 'bundle str > , 2 > , # [doc = " Track errors accumulated during resolving."] pub errors : Option < & 'errors mut Vec < FluentError > > , # [doc = " Makes the resolver bail."] pub dirty : bool , }
};
}
