macro_rules! deps {
    () => {
        ParamsInScope!();
        Enum!();
        ContainerKind!();
        Variant!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a > Enum < 'a > { fn from_syn (node : & 'a DeriveInput , data : & 'a DataEnum) -> Result < Self > { let attrs = attr :: get (& node . attrs) ? ; let scope = ParamsInScope :: new (& node . generics) ; let variants = data . variants . iter () . map (| node | { let mut variant = Variant :: from_syn (node , & scope) ? ; if variant . attrs . display . is_none () && variant . attrs . transparent . is_none () && variant . attrs . fmt . is_none () { variant . attrs . display . clone_from (& attrs . display) ; variant . attrs . transparent = attrs . transparent ; variant . attrs . fmt . clone_from (& attrs . fmt) ; } if let Some (display) = & mut variant . attrs . display { let container = ContainerKind :: from_variant (node) ; display . expand_shorthand (& variant . fields , container) ? ; } Ok (variant) }) . collect :: < Result < _ > > () ? ; Ok (Enum { attrs , ident : node . ident . clone () , generics : & node . generics , variants , }) } }
    };
}

impl_9!();