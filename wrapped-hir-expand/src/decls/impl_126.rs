macro_rules! deps {
    () => {
        ModPath!();
        ExpandDatabase!();
        Display!();
        PathKind!();
        Name!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl ModPath { pub fn from_src (db : & dyn ExpandDatabase , path : ast :: Path , span_for_range : & mut dyn FnMut (:: tt :: TextRange) -> SyntaxContext ,) -> Option < ModPath > { convert_path (db , path , span_for_range) } pub fn from_tt (db : & dyn ExpandDatabase , tt : tt :: TokenTreesView < '_ >) -> Option < ModPath > { convert_path_tt (db , tt) } pub fn from_segments (kind : PathKind , segments : impl IntoIterator < Item = Name >) -> ModPath { let mut segments : SmallVec < Name , 1 > = segments . into_iter () . collect () ; segments . shrink_to_fit () ; ModPath { kind , segments } } # [doc = " Creates a `ModPath` from a `PathKind`, with no extra path segments."] pub const fn from_kind (kind : PathKind) -> ModPath { ModPath { kind , segments : SmallVec :: < Name , 1 > :: new () } } pub fn segments (& self) -> & [Name] { & self . segments } pub fn push_segment (& mut self , segment : Name) { self . segments . push (segment) ; } pub fn pop_segment (& mut self) -> Option < Name > { self . segments . pop () } # [doc = " Returns the number of segments in the path (counting special segments like `$crate` and"] # [doc = " `super`)."] pub fn len (& self) -> usize { self . segments . len () + match self . kind { PathKind :: Plain => 0 , PathKind :: Super (i) => i as usize , PathKind :: Crate => 1 , PathKind :: Abs => 0 , PathKind :: DollarCrate (_) => 1 , } } pub fn textual_len (& self) -> usize { let base = match self . kind { PathKind :: Plain => 0 , PathKind :: SELF => "self" . len () , PathKind :: Super (i) => "super" . len () * i as usize , PathKind :: Crate => "crate" . len () , PathKind :: Abs => 0 , PathKind :: DollarCrate (_) => "$crate" . len () , } ; self . segments () . iter () . map (| segment | segment . as_str () . len ()) . fold (base , core :: ops :: Add :: add) } pub fn is_ident (& self) -> bool { self . as_ident () . is_some () } pub fn is_self (& self) -> bool { self . kind == PathKind :: SELF && self . segments . is_empty () } # [allow (non_snake_case)] pub fn is_Self (& self) -> bool { self . kind == PathKind :: Plain && matches ! (&* self . segments , [name] if * name == sym :: Self_) } # [doc = " If this path is a single identifier, like `foo`, return its name."] pub fn as_ident (& self) -> Option < & Name > { if self . kind != PathKind :: Plain { return None ; } match & * self . segments { [name] => Some (name) , _ => None , } } pub fn display_verbatim < 'a > (& 'a self , db : & 'a dyn crate :: db :: ExpandDatabase ,) -> impl fmt :: Display + 'a { Display { db , path : self , edition : None } } pub fn display < 'a > (& 'a self , db : & 'a dyn crate :: db :: ExpandDatabase , edition : Edition ,) -> impl fmt :: Display + 'a { Display { db , path : self , edition : Some (edition) } } }
    };
}

impl_126!()