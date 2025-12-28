macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! NameClass {
    () => {
        deps!();
        # [doc = " On a first blush, a single `ast::Name` defines a single definition at some"] # [doc = " scope. That is, that, by just looking at the syntactical category, we can"] # [doc = " unambiguously define the semantic category."] # [doc = ""] # [doc = " Sadly, that's not 100% true, there are special cases. To make sure that"] # [doc = " callers handle all the special cases correctly via exhaustive matching, we"] # [doc = " add a [`NameClass`] enum which lists all of them!"] # [doc = ""] # [doc = " A model special case is `None` constant in pattern."] # [derive (Debug)] pub enum NameClass < 'db > { Definition (Definition) , # [doc = " `None` in `if let None = Some(82) {}`."] # [doc = " Syntactically, it is a name, but semantically it is a reference."] ConstReference (Definition) , # [doc = " `field` in `if let Foo { field } = foo`. Here, `ast::Name` both introduces"] # [doc = " a definition into a local scope, and refers to an existing definition."] PatFieldShorthand { local_def : Local , field_ref : Field , adt_subst : GenericSubstitution < 'db > , } , }
    };
}

NameClass!();