macro_rules! deps {
    () => {
        Crate!();
        OpaqueTy!();
        Impl!();
        AnonConst!();
        DefPathDataName!();
        DefPathData!();
        Closure!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl DefPathData { pub fn get_opt_name (& self) -> Option < Symbol > { use self :: DefPathData :: * ; match * self { TypeNs (name) | ValueNs (name) | MacroNs (name) | LifetimeNs (name) | OpaqueLifetime (name) => Some (name) , Impl | ForeignMod | CrateRoot | Use | GlobalAsm | Closure | Ctor | AnonConst | OpaqueTy | AnonAssocTy (..) | SyntheticCoroutineBody | NestedStatic => None , } } fn hashed_symbol (& self) -> Option < Symbol > { use self :: DefPathData :: * ; match * self { TypeNs (name) | ValueNs (name) | MacroNs (name) | LifetimeNs (name) | AnonAssocTy (name) | OpaqueLifetime (name) => Some (name) , Impl | ForeignMod | CrateRoot | Use | GlobalAsm | Closure | Ctor | AnonConst | OpaqueTy | SyntheticCoroutineBody | NestedStatic => None , } } pub fn name (& self) -> DefPathDataName { use self :: DefPathData :: * ; match * self { TypeNs (name) | ValueNs (name) | MacroNs (name) | LifetimeNs (name) | OpaqueLifetime (name) => DefPathDataName :: Named (name) , CrateRoot => DefPathDataName :: Anon { namespace : kw :: Crate } , Impl => DefPathDataName :: Anon { namespace : kw :: Impl } , ForeignMod => DefPathDataName :: Anon { namespace : kw :: Extern } , Use => DefPathDataName :: Anon { namespace : kw :: Use } , GlobalAsm => DefPathDataName :: Anon { namespace : sym :: global_asm } , Closure => DefPathDataName :: Anon { namespace : sym :: closure } , Ctor => DefPathDataName :: Anon { namespace : sym :: constructor } , AnonConst => DefPathDataName :: Anon { namespace : sym :: constant } , OpaqueTy => DefPathDataName :: Anon { namespace : sym :: opaque } , AnonAssocTy (..) => DefPathDataName :: Anon { namespace : sym :: anon_assoc } , SyntheticCoroutineBody => DefPathDataName :: Anon { namespace : sym :: synthetic } , NestedStatic => DefPathDataName :: Anon { namespace : sym :: nested } , } } }
    };
}

impl_97!()