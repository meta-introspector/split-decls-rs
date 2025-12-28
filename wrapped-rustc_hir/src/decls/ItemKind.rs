macro_rules! deps {
    () => {
        Ty!();
        Safety!();
        Mod!();
        InlineAsm!();
        MacroKinds!();
        Impl!();
        Generics!();
        TraitItemId!();
        ForeignItemId!();
        BodyId!();
        VariantData!();
        FnSig!();
        UseKind!();
        EnumDef!();
        GenericBounds!();
        UsePath!();
        Constness!();
    };
}

macro_rules! ItemKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum ItemKind < 'hir > { # [doc = " An `extern crate` item, with optional *original* crate name if the crate was renamed."] # [doc = ""] # [doc = " E.g., `extern crate foo` or `extern crate foo_bar as foo`."] ExternCrate (Option < Symbol > , Ident) , # [doc = " `use foo::bar::*;` or `use foo::bar::baz as quux;`"] # [doc = ""] # [doc = " or just"] # [doc = ""] # [doc = " `use foo::bar::baz;` (with `as baz` implicitly on the right)."] Use (& 'hir UsePath < 'hir > , UseKind) , # [doc = " A `static` item."] Static (Mutability , Ident , & 'hir Ty < 'hir > , BodyId) , # [doc = " A `const` item."] Const (Ident , & 'hir Generics < 'hir > , & 'hir Ty < 'hir > , BodyId) , # [doc = " A function declaration."] Fn { sig : FnSig < 'hir > , ident : Ident , generics : & 'hir Generics < 'hir > , body : BodyId , # [doc = " Whether this function actually has a body."] # [doc = " For functions without a body, `body` is synthesized (to avoid ICEs all over the"] # [doc = " compiler), but that code should never be translated."] has_body : bool , } , # [doc = " A MBE macro definition (`macro_rules!` or `macro`)."] Macro (Ident , & 'hir ast :: MacroDef , MacroKinds) , # [doc = " A module."] Mod (Ident , & 'hir Mod < 'hir >) , # [doc = " An external module, e.g. `extern { .. }`."] ForeignMod { abi : ExternAbi , items : & 'hir [ForeignItemId] } , # [doc = " Module-level inline assembly (from `global_asm!`)."] GlobalAsm { asm : & 'hir InlineAsm < 'hir > , # [doc = " A fake body which stores typeck results for the global asm's sym_fn"] # [doc = " operands, which are represented as path expressions. This body contains"] # [doc = " a single [`ExprKind::InlineAsm`] which points to the asm in the field"] # [doc = " above, and which is typechecked like a inline asm expr just for the"] # [doc = " typeck results."] fake_body : BodyId , } , # [doc = " A type alias, e.g., `type Foo = Bar<u8>`."] TyAlias (Ident , & 'hir Generics < 'hir > , & 'hir Ty < 'hir >) , # [doc = " An enum definition, e.g., `enum Foo<A, B> { C<A>, D<B> }`."] Enum (Ident , & 'hir Generics < 'hir > , EnumDef < 'hir >) , # [doc = " A struct definition, e.g., `struct Foo<A> {x: A}`."] Struct (Ident , & 'hir Generics < 'hir > , VariantData < 'hir >) , # [doc = " A union definition, e.g., `union Foo<A, B> {x: A, y: B}`."] Union (Ident , & 'hir Generics < 'hir > , VariantData < 'hir >) , # [doc = " A trait definition."] Trait (Constness , IsAuto , Safety , Ident , & 'hir Generics < 'hir > , GenericBounds < 'hir > , & 'hir [TraitItemId] ,) , # [doc = " A trait alias."] TraitAlias (Ident , & 'hir Generics < 'hir > , GenericBounds < 'hir >) , # [doc = " An implementation, e.g., `impl<A> Trait for Foo { .. }`."] Impl (Impl < 'hir >) , }
    };
}

ItemKind!();