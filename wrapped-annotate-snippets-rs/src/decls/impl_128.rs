macro_rules! deps {
    () => {
        Title!();
        Id!();
        Element!();
        Report!();
        Group!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < 'a > Title < 'a > { # [doc = " The category for this [`Report`]"] # [doc = ""] # [doc = " Useful for looking searching for more information to resolve the diagnostic."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is considered \"untrusted input\", as such"] # [doc = " all text is passed through a normalization function. Styled text is"] # [doc = " not allowed to be passed to this function."] # [doc = ""] # [doc = " </div>"] pub fn id (mut self , id : impl Into < Cow < 'a , str > >) -> Self { self . id . get_or_insert (Id :: default ()) . id = Some (id . into ()) ; self } # [doc = " Provide a URL for [`Title::id`] for more information on this diagnostic"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " This is only relevant if `id` is present"] # [doc = ""] # [doc = " </div>"] pub fn id_url (mut self , url : impl Into < Cow < 'a , str > >) -> Self { self . id . get_or_insert (Id :: default ()) . url = Some (url . into ()) ; self } # [doc = " Append an [`Element`] that adds context to the [`Title`]"] pub fn element (self , section : impl Into < Element < 'a > >) -> Group < 'a > { Group :: with_title (self) . element (section) } # [doc = " Append [`Element`]s that adds context to the [`Title`]"] pub fn elements (self , sections : impl IntoIterator < Item = impl Into < Element < 'a > > >) -> Group < 'a > { Group :: with_title (self) . elements (sections) } }
    };
}

impl_128!()