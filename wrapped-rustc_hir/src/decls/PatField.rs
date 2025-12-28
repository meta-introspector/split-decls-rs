macro_rules! deps {
    () => {
        Pat!();
    };
}

macro_rules! PatField {
    () => {
        deps!();
        # [doc = " A single field in a struct pattern."] # [doc = ""] # [doc = " Patterns like the fields of Foo `{ x, ref y, ref mut z }`"] # [doc = " are treated the same as` x: x, y: ref y, z: ref mut z`,"] # [doc = " except `is_shorthand` is true."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct PatField < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , # [doc = " The identifier for the field."] pub ident : Ident , # [doc = " The pattern the field is destructured to."] pub pat : & 'hir Pat < 'hir > , pub is_shorthand : bool , pub span : Span , }
    };
}

PatField!()