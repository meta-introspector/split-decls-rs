macro_rules! deps {
    () => {
        Walker!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < C , W : ? Sized > Walker < C > for & mut W where W : Walker < C > , { type Item = W :: Item ; fn walk_next (& mut self , context : C) -> Option < Self :: Item > { (* * self) . walk_next (context) } }
    };
}

impl_58!();