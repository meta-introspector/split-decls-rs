// Generated macro for LoaderContext (enum)
macro_rules! Depcrate_refs_referenceLoaderContext {
() => {
// Module: crate::refs::reference
// Provides: {"LoaderContext"}
// Dependencies: {}
# [doc = " Represents the context that influences how a class may be loaded."] # [derive (Debug , Default)] pub enum LoaderContext < 'any_local , 'a > { # [doc = " There's no extra context that influences how the class should be loaded, and a default"] # [doc = " strategy will be used:"] # [doc = ""] # [doc = " 1. The Thread context will be used to find a ClassLoader to check via Class.forName"] # [doc = " 2. FindClass will be called"] # [default] None , # [doc = " A direct reference to the class loader that should be used (with no fallback to FindClass)"] Loader (& 'a JClassLoader < 'any_local >) , # [doc = " In case we don't have a direct reference, to a `ClassLoader`, the ClassLoader associated"] # [doc = " with this object's Class may be checked"] # [doc = ""] # [doc = " This is used when downcasting, where we can speculate that the object being"] # [doc = " downcast _should_ be associated with the correct `ClassLoader`."] # [doc = ""] # [doc = " The search strategy will be:"] # [doc = " 1. The Thread context will be used to find a ClassLoader to check via Class.forName"] # [doc = " 2. The ClassLoader associated with the object being downcast will be used"] # [doc = " 3. FindClass will be called"] FromObject (& 'a JObject < 'any_local >) , }
};
}
