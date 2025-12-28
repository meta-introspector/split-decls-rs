macro_rules! deps {
    () => {
        Definition!();
        NameClass!();
    };
}

macro_rules! NameRefClass {
    () => {
        deps!();
        # [doc = " This is similar to [`NameClass`], but works for [`ast::NameRef`] rather than"] # [doc = " for [`ast::Name`]. Similarly, what looks like a reference in syntax is a"] # [doc = " reference most of the time, but there are a couple of annoying exceptions."] # [doc = ""] # [doc = " A model special case is field shorthand syntax, which uses a single"] # [doc = " reference to point to two different defs."] # [derive (Debug)] pub enum NameRefClass < 'db > { Definition (Definition , Option < GenericSubstitution < 'db > >) , FieldShorthand { local_ref : Local , field_ref : Field , adt_subst : GenericSubstitution < 'db > , } , # [doc = " The specific situation where we have an extern crate decl without a rename"] # [doc = " Here we have both a declaration and a reference."] # [doc = " ```rs"] # [doc = " extern crate foo;"] # [doc = " ```"] ExternCrateShorthand { decl : ExternCrateDecl , krate : Crate , } , }
    };
}

NameRefClass!();