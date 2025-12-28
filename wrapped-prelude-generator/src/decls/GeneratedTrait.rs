macro_rules! deps {
    () => {
        GeneratedTraitMethod!();
        GeneratedAssociatedType!();
    };
}

macro_rules! GeneratedTrait {
    () => {
        deps!();
        # [derive (Debug)] pub struct GeneratedTrait { pub name : String , pub generics : Option < TokenStream > , pub where_clause : Option < TokenStream > , pub visibility : Option < TokenStream > , pub methods : Vec < GeneratedTraitMethod > , pub associated_types : Vec < GeneratedAssociatedType > , pub supertraits : Vec < TokenStream > , }
    };
}

GeneratedTrait!();