macro_rules! deps {
    () => {
        Name!();
        Id!();
        ResourceName!();
        ResourceNameOrId!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        impl ResourceNameOrId { # [doc = " Converts to an option of name."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn name (self) -> Option < ResourceName > { match self { Self :: Name (name) => Some (name) , _ => None , } } # [doc = " Converts to an option of ID."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn id (self) -> Option < u16 > { match self { Self :: Id (id) => Some (id) , _ => None , } } }
    };
}

impl_729!();