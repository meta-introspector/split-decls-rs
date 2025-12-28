macro_rules! deps {
    () => {
        Title!();
        Level!();
        Stylesheet!();
        LevelInner!();
        Message!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > Level < 'a > { # [doc = " For the primary, or root cause, [`Group`][crate::Group] (the first) in a [`Report`][crate::Report]"] # [doc = ""] # [doc = " See [`Group::with_title`][crate::Group::with_title]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is considered \"untrusted input\", as such"] # [doc = " all text is passed through a normalization function. Styled text is"] # [doc = " not allowed to be passed to this function."] # [doc = ""] # [doc = " </div>"] pub fn primary_title (self , text : impl Into < Cow < 'a , str > >) -> Title < 'a > { Title { level : self , id : None , text : text . into () , allows_styling : false , } } # [doc = " For any secondary, or context, [`Group`][crate::Group]s (subsequent) in a [`Report`][crate::Report]"] # [doc = ""] # [doc = " See [`Group::with_title`][crate::Group::with_title]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is allowed to be styled, as such all"] # [doc = " text is considered \"trusted input\" and has no normalizations applied to"] # [doc = " it. [`normalize_untrusted_str`](crate::normalize_untrusted_str) can be"] # [doc = " used to normalize untrusted text before it is passed to this function."] # [doc = ""] # [doc = " </div>"] pub fn secondary_title (self , text : impl Into < Cow < 'a , str > >) -> Title < 'a > { Title { level : self , id : None , text : text . into () , allows_styling : true , } } # [doc = " A text [`Element`][crate::Element] in a [`Group`][crate::Group]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is allowed to be styled, as such all"] # [doc = " text is considered \"trusted input\" and has no normalizations applied to"] # [doc = " it. [`normalize_untrusted_str`](crate::normalize_untrusted_str) can be"] # [doc = " used to normalize untrusted text before it is passed to this function."] # [doc = ""] # [doc = " </div>"] pub fn message (self , text : impl Into < Cow < 'a , str > >) -> Message < 'a > { Message { level : self , text : text . into () , } } pub (crate) fn as_str (& 'a self) -> & 'a str { match (& self . name , self . level) { (Some (Some (name)) , _) => name . as_ref () , (Some (None) , _) => "" , (None , LevelInner :: Error) => ERROR_TXT , (None , LevelInner :: Warning) => WARNING_TXT , (None , LevelInner :: Info) => INFO_TXT , (None , LevelInner :: Note) => NOTE_TXT , (None , LevelInner :: Help) => HELP_TXT , } } pub (crate) fn style (& self , stylesheet : & Stylesheet) -> Style { self . level . style (stylesheet) } }
    };
}

impl_7!()