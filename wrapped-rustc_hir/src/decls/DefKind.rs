macro_rules! deps {
    () => {
        Mod!();
        Impl!();
        DefPathData!();
        Safety!();
        CtorKind!();
        CtorOf!();
        Lifetime!();
        AnonConst!();
        Variant!();
        MacroKinds!();
        OpaqueTy!();
        Closure!();
    };
}

macro_rules! DefKind {
    () => {
        deps!();
        # [doc = " What kind of definition something is; e.g., `mod` vs `struct`."] # [doc = " `enum DefPathData` may need to be updated if a new variant is added here."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum DefKind { Mod , # [doc = " Refers to the struct itself, [`DefKind::Ctor`] refers to its constructor if it exists."] Struct , Union , Enum , # [doc = " Refers to the variant itself, [`DefKind::Ctor`] refers to its constructor if it exists."] Variant , Trait , # [doc = " Type alias: `type Foo = Bar;`"] TyAlias , # [doc = " Type from an `extern` block."] ForeignTy , # [doc = " Trait alias: `trait IntIterator = Iterator<Item = i32>;`"] TraitAlias , # [doc = " Associated type: `trait MyTrait { type Assoc; }`"] AssocTy , # [doc = " Type parameter: the `T` in `struct Vec<T> { ... }`"] TyParam , Fn , Const , # [doc = " Constant generic parameter: `struct Foo<const N: usize> { ... }`"] ConstParam , Static { # [doc = " Whether it's a `unsafe static`, `safe static` (inside extern only) or just a `static`."] safety : hir :: Safety , # [doc = " Whether it's a `static mut` or just a `static`."] mutability : ast :: Mutability , # [doc = " Whether it's an anonymous static generated for nested allocations."] nested : bool , } , # [doc = " Refers to the struct or enum variant's constructor."] # [doc = ""] # [doc = " The reason `Ctor` exists in addition to [`DefKind::Struct`] and"] # [doc = " [`DefKind::Variant`] is because structs and enum variants exist"] # [doc = " in the *type* namespace, whereas struct and enum variant *constructors*"] # [doc = " exist in the *value* namespace."] # [doc = ""] # [doc = " You may wonder why enum variants exist in the type namespace as opposed"] # [doc = " to the value namespace. Check out [RFC 2593] for intuition on why that is."] # [doc = ""] # [doc = " [RFC 2593]: https://github.com/rust-lang/rfcs/pull/2593"] Ctor (CtorOf , CtorKind) , # [doc = " Associated function: `impl MyStruct { fn associated() {} }`"] # [doc = " or `trait Foo { fn associated() {} }`"] AssocFn , # [doc = " Associated constant: `trait MyTrait { const ASSOC: usize; }`"] AssocConst , Macro (MacroKinds) , ExternCrate , Use , # [doc = " An `extern` block."] ForeignMod , # [doc = " Anonymous constant, e.g. the `1 + 2` in `[u8; 1 + 2]`."] # [doc = ""] # [doc = " Not all anon-consts are actually still relevant in the HIR. We lower"] # [doc = " trivial const-arguments directly to `hir::ConstArgKind::Path`, at which"] # [doc = " point the definition for the anon-const ends up unused and incomplete."] # [doc = ""] # [doc = " We do not provide any a `Span` for the definition and pretty much all other"] # [doc = " queries also ICE when using this `DefId`. Given that the `DefId` of such"] # [doc = " constants should only be reachable by iterating all definitions of a"] # [doc = " given crate, you should not have to worry about this."] AnonConst , # [doc = " An inline constant, e.g. `const { 1 + 2 }`"] InlineConst , # [doc = " Opaque type, aka `impl Trait`."] OpaqueTy , # [doc = " A field in a struct, enum or union. e.g."] # [doc = " - `bar` in `struct Foo { bar: u8 }`"] # [doc = " - `Foo::Bar::0` in `enum Foo { Bar(u8) }`"] Field , # [doc = " Lifetime parameter: the `'a` in `struct Foo<'a> { ... }`"] LifetimeParam , # [doc = " A use of `global_asm!`."] GlobalAsm , Impl { of_trait : bool , } , # [doc = " A closure, coroutine, or coroutine-closure."] # [doc = ""] # [doc = " These are all represented with the same `ExprKind::Closure` in the AST and HIR,"] # [doc = " which makes it difficult to distinguish these during def collection. Therefore,"] # [doc = " we treat them all the same, and code which needs to distinguish them can match"] # [doc = " or `hir::ClosureKind` or `type_of`."] Closure , # [doc = " The definition of a synthetic coroutine body created by the lowering of a"] # [doc = " coroutine-closure, such as an async closure."] SyntheticCoroutineBody , }
    };
}

DefKind!();