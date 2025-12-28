macro_rules! deps {
    () => {
        AnonConst!();
        OpaqueTy!();
        DefKind!();
        Impl!();
        Closure!();
    };
}

macro_rules! DefPathData {
    () => {
        deps!();
        # [doc = " New variants should only be added in synchronization with `enum DefKind`."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Encodable , Decodable)] pub enum DefPathData { # [doc = " The crate root (marker)."] CrateRoot , # [doc = " An impl."] Impl , # [doc = " An `extern` block."] ForeignMod , # [doc = " A `use` item."] Use , # [doc = " A global asm item."] GlobalAsm , # [doc = " Something in the type namespace."] TypeNs (Symbol) , # [doc = " Something in the value namespace."] ValueNs (Symbol) , # [doc = " Something in the macro namespace."] MacroNs (Symbol) , # [doc = " Something in the lifetime namespace."] LifetimeNs (Symbol) , # [doc = " A closure expression."] Closure , # [doc = " Implicit constructor for a unit or tuple-like struct or enum variant."] Ctor , # [doc = " A constant expression (see `{ast,hir}::AnonConst`)."] AnonConst , # [doc = " An existential `impl Trait` type node."] # [doc = " Argument position `impl Trait` have a `TypeNs` with their pretty-printed name."] OpaqueTy , # [doc = " Used for remapped captured lifetimes in an existential `impl Trait` type node."] OpaqueLifetime (Symbol) , # [doc = " An anonymous associated type from an RPITIT. The symbol refers to the name of the method"] # [doc = " that defined the type."] AnonAssocTy (Symbol) , # [doc = " A synthetic body for a coroutine's by-move body."] SyntheticCoroutineBody , # [doc = " Additional static data referred to by a static."] NestedStatic , }
    };
}

DefPathData!();