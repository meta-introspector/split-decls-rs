macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        # [doc = " Attribute matching specific methods"] impl Stack { # [doc = " Creates a new container to store match outcomes for all attribute matches."] # [doc = ""] # [doc = " ### Panics"] # [doc = ""] # [doc = " If attributes aren't configured."] pub fn attribute_matches (& self) -> gix_attributes :: search :: Outcome { let mut out = gix_attributes :: search :: Outcome :: default () ; out . initialize (& self . state . attributes_or_panic () . collection) ; out } # [doc = " Creates a new container to store match outcomes for the given attributes."] # [doc = ""] # [doc = " ### Panics"] # [doc = ""] # [doc = " If attributes aren't configured."] pub fn selected_attribute_matches < 'a > (& self , given : impl IntoIterator < Item = impl Into < & 'a str > > ,) -> gix_attributes :: search :: Outcome { let mut out = gix_attributes :: search :: Outcome :: default () ; out . initialize_with_selection (& self . state . attributes_or_panic () . collection , given . into_iter () . map (Into :: into) ,) ; out } # [doc = " Return the metadata collection that enables initializing attribute match outcomes as done in"] # [doc = " [`attribute_matches()`][Stack::attribute_matches()] or [`selected_attribute_matches()`][Stack::selected_attribute_matches()]"] # [doc = ""] # [doc = " ### Panics"] # [doc = ""] # [doc = " If attributes aren't configured."] pub fn attributes_collection (& self) -> & gix_attributes :: search :: MetadataCollection { & self . state . attributes_or_panic () . collection } }
    };
}

impl_28!();