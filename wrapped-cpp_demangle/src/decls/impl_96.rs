macro_rules! deps {
    () => {
        CvQualifiers!();
        NestedName!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl NestedName { # [doc = " Get the CV-qualifiers for this name."] pub fn cv_qualifiers (& self) -> Option < & CvQualifiers > { match * self { NestedName :: Unqualified (ref q , ..) | NestedName :: Template (ref q , ..) => Some (q) , _ => None , } } # [doc = " Get the ref-qualifier for this name, if one exists."] pub fn ref_qualifier (& self) -> Option < & RefQualifier > { match * self { NestedName :: Unqualified (_ , Some (ref r) , ..) | NestedName :: Template (_ , Some (ref r) , ..) => Some (r) , _ => None , } } fn prefix (& self) -> Option < & PrefixHandle > { match * self { NestedName :: Unqualified (_ , _ , ref p , _) | NestedName :: UnqualifiedExplicitObject (ref p , ..) => p . as_ref () , NestedName :: Template (_ , _ , ref p) | NestedName :: TemplateExplicitObject (ref p , _) => { Some (p) } } } # [doc = " Check to see if the object has an explicit named parameter."] pub fn has_explicit_obj_param (& self) -> bool { match * self { NestedName :: UnqualifiedExplicitObject (_ , _ , ref _e) | NestedName :: TemplateExplicitObject (_ , ref _e) => true , _ => false , } } }
    };
}

impl_96!()