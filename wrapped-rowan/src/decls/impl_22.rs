macro_rules! deps {
    () => {
        Direction!();
        SyntaxElement!();
        Green!();
        SyntaxToken!();
        SyntaxNode!();
        NodeData!();
        SyntaxKind!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl SyntaxToken { fn new (green : & GreenTokenData , parent : SyntaxNode , index : u32 , offset : TextSize ,) -> SyntaxToken { let mutable = parent . data () . mutable ; let green = Green :: Token { ptr : green . into () } ; SyntaxToken { ptr : NodeData :: new (Some (parent) , index , offset , green , mutable) } } # [inline] fn data (& self) -> & NodeData { unsafe { self . ptr . as_ref () } } # [inline] fn can_take_ptr (& self) -> bool { self . data () . rc . get () == 1 && ! self . data () . mutable } # [inline] fn take_ptr (self) -> ptr :: NonNull < NodeData > { assert ! (self . can_take_ptr ()) ; let ret = self . ptr ; std :: mem :: forget (self) ; ret } pub fn replace_with (& self , replacement : GreenToken) -> GreenNode { assert_eq ! (self . kind () , replacement . kind ()) ; let parent = self . parent () . unwrap () ; let me : u32 = self . data () . index () ; let new_parent = parent . green_ref () . replace_child (me as usize , replacement . into ()) ; parent . replace_with (new_parent) } # [inline] pub fn kind (& self) -> SyntaxKind { self . data () . kind () } # [inline] pub fn text_range (& self) -> TextRange { self . data () . text_range () } # [inline] pub fn index (& self) -> usize { self . data () . index () as usize } # [inline] pub fn text (& self) -> & str { match self . data () . green () . as_token () { Some (it) => it . text () , None => { debug_assert ! (false , "corrupted tree: a node thinks it is a token: {:?}" , self . data () . green () . as_node () . unwrap () . to_string ()) ; "" } } } # [inline] pub fn green (& self) -> & GreenTokenData { self . data () . green () . into_token () . unwrap () } # [inline] pub fn parent (& self) -> Option < SyntaxNode > { self . data () . parent_node () } # [inline] pub fn ancestors (& self) -> impl Iterator < Item = SyntaxNode > + use < > { std :: iter :: successors (self . parent () , SyntaxNode :: parent) } pub fn next_sibling_or_token (& self) -> Option < SyntaxElement > { self . data () . next_sibling_or_token () } pub fn next_sibling_or_token_by_kind (& self , matcher : & impl Fn (SyntaxKind) -> bool ,) -> Option < SyntaxElement > { self . data () . next_sibling_or_token_by_kind (matcher) } pub fn prev_sibling_or_token (& self) -> Option < SyntaxElement > { self . data () . prev_sibling_or_token () } # [inline] pub fn siblings_with_tokens (& self , direction : Direction ,) -> impl Iterator < Item = SyntaxElement > + use < > { let me : SyntaxElement = self . clone () . into () ; iter :: successors (Some (me) , move | el | match direction { Direction :: Next => el . next_sibling_or_token () , Direction :: Prev => el . prev_sibling_or_token () , }) } pub fn next_token (& self) -> Option < SyntaxToken > { match self . next_sibling_or_token () { Some (element) => element . first_token () , None => self . ancestors () . find_map (| it | it . next_sibling_or_token ()) . and_then (| element | element . first_token ()) , } } pub fn prev_token (& self) -> Option < SyntaxToken > { match self . prev_sibling_or_token () { Some (element) => element . last_token () , None => self . ancestors () . find_map (| it | it . prev_sibling_or_token ()) . and_then (| element | element . last_token ()) , } } pub fn detach (& self) { assert ! (self . data () . mutable , "immutable tree: {}" , self) ; self . data () . detach () } }
    };
}

impl_22!();