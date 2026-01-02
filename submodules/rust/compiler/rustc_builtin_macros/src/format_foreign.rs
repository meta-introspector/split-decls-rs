mkmod!{printf, { 
                getname!(printf);
                getsrc!(printf);
                getpath!(printf);
                get_deps!(printf);
                get_crates!(printf);
                mkinclude!(printf);
                mkuse!{use rustc_span :: InnerSpan ;}
mkuse!{use super :: strcursor :: StrCursor as Cur ;}
mkitem!{mkenum!{# [doc = " Represents a single `printf`-style substitution."] # [derive (Clone , PartialEq , Debug)] pub (crate) enum Substitution < 'a > { # [doc = " A formatted output substitution with its internal byte offset."] Format (Format < 'a >) , # [doc = " A literal `%%` escape, with its start and end indices."] Escape ((usize , usize)) , }}}
mkitem!{mkimpl!{impl ToString for Substitution < '_ > { fn to_string (& self) -> String { match self { Substitution :: Format (fmt) => fmt . span . into () , Substitution :: Escape (_) => "%%" . into () , } } }}}
mkitem!{mkimpl!{impl Substitution < '_ > { pub (crate) fn position (& self) -> InnerSpan { match self { Substitution :: Format (fmt) => fmt . position , & Substitution :: Escape ((start , end)) => InnerSpan :: new (start , end) , } } pub (crate) fn set_position (& mut self , start : usize , end : usize) { match self { Substitution :: Format (fmt) => fmt . position = InnerSpan :: new (start , end) , Substitution :: Escape (pos) => * pos = (start , end) , } } # [doc = " Translate this substitution into an equivalent Rust formatting directive."] # [doc = ""] # [doc = " This ignores cases where the substitution does not have an exact equivalent, or where"] # [doc = " the substitution would be unnecessary."] pub (crate) fn translate (& self) -> Result < String , Option < String > > { match self { Substitution :: Format (fmt) => fmt . translate () , Substitution :: Escape (_) => Err (None) , } } }}}
mkitem!{mkstruct!{# [derive (Clone , PartialEq , Debug)] # [doc = " A single `printf`-style formatting directive."] pub (crate) struct Format < 'a > { # [doc = " The entire original formatting directive."] span : & 'a str , # [doc = " The (1-based) parameter to be converted."] parameter : Option < u16 > , # [doc = " Formatting flags."] flags : & 'a str , # [doc = " Minimum width of the output."] width : Option < Num > , # [doc = " Precision of the conversion."] precision : Option < Num > , # [doc = " Length modifier for the conversion."] length : Option < & 'a str > , # [doc = " Type of parameter being converted."] type_ : & 'a str , # [doc = " Byte offset for the start and end of this formatting directive."] position : InnerSpan , }}}
mkitem!{mkimpl!{impl Format < '_ > { # [doc = " Translate this directive into an equivalent Rust formatting directive."] # [doc = ""] # [doc = " Returns `Err` in cases where the `printf` directive does not have an exact Rust"] # [doc = " equivalent, rather than guessing."] pub (crate) fn translate (& self) -> Result < String , Option < String > > { use std :: fmt :: Write ; let (c_alt , c_zero , c_left , c_plus) = { let mut c_alt = false ; let mut c_zero = false ; let mut c_left = false ; let mut c_plus = false ; for c in self . flags . chars () { match c { '#' => c_alt = true , '0' => c_zero = true , '-' => c_left = true , '+' => c_plus = true , _ => { return Err (Some (format ! ("the flag `{c}` is unknown or unsupported"))) ; } } } (c_alt , c_zero , c_left , c_plus) } ; let fill = c_zero . then_some ("0") ; let align = c_left . then_some ("<") ; let sign = c_plus . then_some ("+") ; let alt = c_alt ; let width = match self . width { Some (Num :: Next) => { return Err (Some ("you have to use a positional or named parameter for the width" . to_string () ,)) ; } w @ Some (Num :: Arg (_)) => w , w @ Some (Num :: Num (_)) => w , None => None , } ; let precision = self . precision ; let (type_ , use_zero_fill , is_int) = match self . type_ { "d" | "i" | "u" => (None , true , true) , "f" | "F" => (None , false , false) , "s" | "c" => (None , false , false) , "e" | "E" => (Some (self . type_) , true , false) , "x" | "X" | "o" => (Some (self . type_) , true , true) , "p" => (Some (self . type_) , false , true) , "g" => (Some ("e") , true , false) , "G" => (Some ("E") , true , false) , _ => { return Err (Some (format ! ("the conversion specifier `{}` is unknown or unsupported" , self . type_))) ; } } ; let (fill , width , precision) = match (is_int , width , precision) { (true , Some (_) , Some (_)) => { return Err (Some ("width and precision cannot both be specified for integer conversions" . to_string () ,)) ; } (true , None , Some (p)) => (Some ("0") , Some (p) , None) , (true , w , None) => (fill , w , None) , (false , w , p) => (fill , w , p) , } ; let align = match (self . type_ , width . is_some () , align . is_some ()) { ("s" , true , false) => Some (">") , _ => align , } ; let (fill , zero_fill) = match (fill , use_zero_fill) { (Some ("0") , true) => (None , true) , (fill , _) => (fill , false) , } ; let alt = match type_ { Some ("x" | "X") => alt , _ => false , } ; let has_options = fill . is_some () || align . is_some () || sign . is_some () || alt || zero_fill || width . is_some () || precision . is_some () || type_ . is_some () ; let cap = self . span . len () + if has_options { 2 } else { 0 } ; let mut s = String :: with_capacity (cap) ; s . push ('{') ; if let Some (arg) = self . parameter { match write ! (s , "{}" , match arg . checked_sub (1) { Some (a) => a , None => return Err (None) , }) { Err (_) => return Err (None) , _ => { } } } if has_options { s . push (':') ; let align = if let Some (fill) = fill { s . push_str (fill) ; align . or (Some (">")) } else { align } ; if let Some (align) = align { s . push_str (align) ; } if let Some (sign) = sign { s . push_str (sign) ; } if alt { s . push ('#') ; } if zero_fill { s . push ('0') ; } if let Some (width) = width { match width . translate (& mut s) { Err (_) => return Err (None) , _ => { } } } if let Some (precision) = precision { s . push ('.') ; match precision . translate (& mut s) { Err (_) => return Err (None) , _ => { } } } if let Some (type_) = type_ { s . push_str (type_) ; } } s . push ('}') ; Ok (s) } }}}
mkitem!{mkenum!{# [doc = " A general number used in a `printf` formatting directive."] # [derive (Copy , Clone , PartialEq , Debug)] enum Num { # [doc = " A specific, fixed value."] Num (u16) , # [doc = " The value is derived from a positional argument."] Arg (u16) , # [doc = " The value is derived from the \"next\" unconverted argument."] Next , }}}
mkitem!{mkimpl!{impl Num { fn from_str (s : & str , arg : Option < & str >) -> Option < Self > { if let Some (arg) = arg { arg . parse () . ok () . map (| arg | Num :: Arg (arg)) } else if s == "*" { Some (Num :: Next) } else { s . parse () . ok () . map (| num | Num :: Num (num)) } } fn translate (& self , s : & mut String) -> std :: fmt :: Result { use std :: fmt :: Write ; match * self { Num :: Num (n) => write ! (s , "{n}") , Num :: Arg (n) => { let n = n . checked_sub (1) . ok_or (std :: fmt :: Error) ? ; write ! (s , "{n}$") } Num :: Next => write ! (s , "*") , } } }}}

macro_rules! iter_subs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_subs in module {}", module_path!());
    };
}

mkfn!{
    iter_subs_introspect!();
    # [doc = " Returns an iterator over all substitutions in a given string."] pub (crate) fn iter_subs (s : & str , start_pos : usize) -> Substitutions < '_ > { Substitutions { s , pos : start_pos } }
}
mkitem!{mkstruct!{# [doc = " Iterator over substitutions in a string."] pub (crate) struct Substitutions < 'a > { s : & 'a str , pos : usize , }}}
mkitem!{mkimpl!{impl < 'a > Iterator for Substitutions < 'a > { type Item = Substitution < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let (mut sub , tail) = parse_next_substitution (self . s) ? ; self . s = tail ; let InnerSpan { start , end } = sub . position () ; sub . set_position (start + self . pos , end + self . pos) ; self . pos += end ; Some (sub) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . s . len () / 2)) } }}}
mkitem!{mkenum!{enum State { Start , Flags , Width , WidthArg , Prec , PrecInner , Length , Type , }}}

macro_rules! parse_next_substitution_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_next_substitution in module {}", module_path!());
    };
}

mkfn!{
    parse_next_substitution_introspect!();
    # [doc = " Parse the next substitution from the input string."] fn parse_next_substitution (s : & str) -> Option < (Substitution < '_ > , & str) > { use self :: State :: * ; let at = { let start = s . find ('%') ? ; if let '%' = s [start + 1 ..] . chars () . next () ? { return Some ((Substitution :: Escape ((start , start + 2)) , & s [start + 2 ..])) ; } Cur :: new_at (s , start) } ; let start = at ; let mut at = at . at_next_cp () ? ; let (mut c , mut next) = at . next_cp () ? ; macro_rules ! move_to { ($ cur : expr) => { { at = $ cur ; let (c_ , next_) = at . next_cp () ?; c = c_ ; next = next_ ; } } ; } let fallback = move | | { Some ((Substitution :: Format (Format { span : start . slice_between (next) . unwrap () , parameter : None , flags : "" , width : None , precision : None , length : None , type_ : at . slice_between (next) . unwrap () , position : InnerSpan :: new (start . at , next . at) , }) , next . slice_after () ,)) } ; let mut state = Start ; let mut parameter : Option < u16 > = None ; let mut flags : & str = "" ; let mut width : Option < Num > = None ; let mut precision : Option < Num > = None ; let mut length : Option < & str > = None ; let mut type_ : & str = "" ; let end : Cur < '_ > ; if let Start = state { match c { '1' ..= '9' => { let end = at_next_cp_while (next , char :: is_ascii_digit) ; match end . next_cp () { Some (('$' , end2)) => { state = Flags ; parameter = at . slice_between (end) . unwrap () . parse () . ok () ; move_to ! (end2) ; } Some (_) => { state = Prec ; parameter = None ; flags = "" ; width = at . slice_between (end) . and_then (| num | Num :: from_str (num , None)) ; if width . is_none () { return fallback () ; } move_to ! (end) ; } None => return fallback () , } } _ => { state = Flags ; parameter = None ; move_to ! (at) ; } } } if let Flags = state { let end = at_next_cp_while (at , is_flag) ; state = Width ; flags = at . slice_between (end) . unwrap () ; move_to ! (end) ; } if let Width = state { match c { '*' => { state = WidthArg ; move_to ! (next) ; } '1' ..= '9' => { let end = at_next_cp_while (next , char :: is_ascii_digit) ; state = Prec ; width = at . slice_between (end) . and_then (| num | Num :: from_str (num , None)) ; if width . is_none () { return fallback () ; } move_to ! (end) ; } _ => { state = Prec ; width = None ; move_to ! (at) ; } } } if let WidthArg = state { let end = at_next_cp_while (at , char :: is_ascii_digit) ; match end . next_cp () { Some (('$' , end2)) => { state = Prec ; width = Num :: from_str ("" , at . slice_between (end)) ; move_to ! (end2) ; } _ => { state = Prec ; width = Some (Num :: Next) ; move_to ! (end) ; } } } if let Prec = state { match c { '.' => { state = PrecInner ; move_to ! (next) ; } _ => { state = Length ; precision = None ; move_to ! (at) ; } } } if let PrecInner = state { match c { '*' => { let end = at_next_cp_while (next , char :: is_ascii_digit) ; match end . next_cp () { Some (('$' , end2)) => { state = Length ; precision = Num :: from_str ("*" , next . slice_between (end)) ; move_to ! (end2) ; } _ => { state = Length ; precision = Some (Num :: Next) ; move_to ! (end) ; } } } '0' ..= '9' => { let end = at_next_cp_while (next , char :: is_ascii_digit) ; state = Length ; precision = at . slice_between (end) . and_then (| num | Num :: from_str (num , None)) ; move_to ! (end) ; } _ => return fallback () , } } if let Length = state { let c1_next1 = next . next_cp () ; match (c , c1_next1) { ('h' , Some (('h' , next1))) | ('l' , Some (('l' , next1))) => { state = Type ; length = Some (at . slice_between (next1) . unwrap ()) ; move_to ! (next1) ; } ('h' | 'l' | 'L' | 'z' | 'j' | 't' | 'q' , _) => { state = Type ; length = Some (at . slice_between (next) . unwrap ()) ; move_to ! (next) ; } ('I' , _) => { let end = next . at_next_cp () . and_then (| end | end . at_next_cp ()) . map (| end | (next . slice_between (end) . unwrap () , end)) ; let end = match end { Some (("32" | "64" , end)) => end , _ => next , } ; state = Type ; length = Some (at . slice_between (end) . unwrap ()) ; move_to ! (end) ; } _ => { state = Type ; length = None ; move_to ! (at) ; } } } if let Type = state { type_ = at . slice_between (next) . unwrap () ; at = next ; } let _ = c ; end = at ; let position = InnerSpan :: new (start . at , end . at) ; let f = Format { span : start . slice_between (end) . unwrap () , parameter , flags , width , precision , length , type_ , position , } ; Some ((Substitution :: Format (f) , end . slice_after ())) }
}

macro_rules! at_next_cp_while_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function at_next_cp_while in module {}", module_path!());
    };
}

mkfn!{
    at_next_cp_while_introspect!();
    fn at_next_cp_while < F > (mut cur : Cur < '_ > , mut pred : F) -> Cur < '_ > where F : FnMut (& char) -> bool , { loop { match cur . next_cp () { Some ((c , next)) => { if pred (& c) { cur = next ; } else { return cur ; } } None => return cur , } } }
}

macro_rules! is_flag_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_flag in module {}", module_path!());
    };
}

mkfn!{
    is_flag_introspect!();
    fn is_flag (c : & char) -> bool { matches ! (c , '0' | '-' | '+' | ' ' | '#' | '\'') }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }} 
            }}
mkmod!{shell, { 
                getname!(shell);
                getsrc!(shell);
                getpath!(shell);
                get_deps!(shell);
                get_crates!(shell);
                mkinclude!(shell);
                mkuse!{use rustc_span :: InnerSpan ;}
mkuse!{use super :: strcursor :: StrCursor as Cur ;}
mkitem!{mkenum!{# [derive (Clone , PartialEq , Debug)] pub (crate) enum Substitution < 'a > { Ordinal (u8 , (usize , usize)) , Name (& 'a str , (usize , usize)) , Escape ((usize , usize)) , }}}
mkitem!{mkimpl!{impl ToString for Substitution < '_ > { fn to_string (& self) -> String { match self { Substitution :: Ordinal (n , _) => format ! ("${n}") , Substitution :: Name (n , _) => format ! ("${n}") , Substitution :: Escape (_) => "$$" . into () , } } }}}
mkitem!{mkimpl!{impl Substitution < '_ > { pub (crate) fn position (& self) -> InnerSpan { let (Self :: Ordinal (_ , pos) | Self :: Name (_ , pos) | Self :: Escape (pos)) = self ; InnerSpan :: new (pos . 0 , pos . 1) } fn set_position (& mut self , start : usize , end : usize) { let (Self :: Ordinal (_ , pos) | Self :: Name (_ , pos) | Self :: Escape (pos)) = self ; * pos = (start , end) ; } pub (crate) fn translate (& self) -> Result < String , Option < String > > { match self { Substitution :: Ordinal (n , _) => Ok (format ! ("{{{}}}" , n)) , Substitution :: Name (n , _) => Ok (format ! ("{{{}}}" , n)) , Substitution :: Escape (_) => Err (None) , } } }}}

macro_rules! iter_subs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_subs in module {}", module_path!());
    };
}

mkfn!{
    iter_subs_introspect!();
    # [doc = " Returns an iterator over all substitutions in a given string."] pub (crate) fn iter_subs (s : & str , start_pos : usize) -> Substitutions < '_ > { Substitutions { s , pos : start_pos } }
}
mkitem!{mkstruct!{# [doc = " Iterator over substitutions in a string."] pub (crate) struct Substitutions < 'a > { s : & 'a str , pos : usize , }}}
mkitem!{mkimpl!{impl < 'a > Iterator for Substitutions < 'a > { type Item = Substitution < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let (mut sub , tail) = parse_next_substitution (self . s) ? ; self . s = tail ; let InnerSpan { start , end } = sub . position () ; sub . set_position (start + self . pos , end + self . pos) ; self . pos += end ; Some (sub) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . s . len ())) } }}}

macro_rules! parse_next_substitution_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_next_substitution in module {}", module_path!());
    };
}

mkfn!{
    parse_next_substitution_introspect!();
    # [doc = " Parse the next substitution from the input string."] fn parse_next_substitution (s : & str) -> Option < (Substitution < '_ > , & str) > { let at = { let start = s . find ('$') ? ; match s [start + 1 ..] . chars () . next () ? { '$' => return Some ((Substitution :: Escape ((start , start + 2)) , & s [start + 2 ..])) , c @ '0' ..= '9' => { let n = (c as u8) - b'0' ; return Some ((Substitution :: Ordinal (n , (start , start + 2)) , & s [start + 2 ..])) ; } _ => { } } Cur :: new_at (s , start) } ; let at = at . at_next_cp () ? ; let (c , inner) = at . next_cp () ? ; if ! is_ident_head (c) { None } else { let end = at_next_cp_while (inner , is_ident_tail) ; let slice = at . slice_between (end) . unwrap () ; let start = at . at - 1 ; let end_pos = at . at + slice . len () ; Some ((Substitution :: Name (slice , (start , end_pos)) , end . slice_after ())) } }
}

macro_rules! at_next_cp_while_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function at_next_cp_while in module {}", module_path!());
    };
}

mkfn!{
    at_next_cp_while_introspect!();
    fn at_next_cp_while < F > (mut cur : Cur < '_ > , mut pred : F) -> Cur < '_ > where F : FnMut (char) -> bool , { loop { match cur . next_cp () { Some ((c , next)) => { if pred (c) { cur = next ; } else { return cur ; } } None => return cur , } } }
}

macro_rules! is_ident_head_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ident_head in module {}", module_path!());
    };
}

mkfn!{
    is_ident_head_introspect!();
    fn is_ident_head (c : char) -> bool { c . is_ascii_alphabetic () || c == '_' }
}

macro_rules! is_ident_tail_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ident_tail in module {}", module_path!());
    };
}

mkfn!{
    is_ident_tail_introspect!();
    fn is_ident_tail (c : char) -> bool { c . is_ascii_alphanumeric () || c == '_' }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }} 
            }}
mkmod!{strcursor, { 
                getname!(strcursor);
                getsrc!(strcursor);
                getpath!(strcursor);
                get_deps!(strcursor);
                get_crates!(strcursor);
                mkinclude!(strcursor);
                mkitem!{mkstruct!{pub (crate) struct StrCursor < 'a > { s : & 'a str , pub at : usize , }}}
mkitem!{mkimpl!{impl < 'a > StrCursor < 'a > { pub (crate) fn new_at (s : & 'a str , at : usize) -> StrCursor < 'a > { StrCursor { s , at } } pub (crate) fn at_next_cp (mut self) -> Option < StrCursor < 'a > > { match self . try_seek_right_cp () { true => Some (self) , false => None , } } pub (crate) fn next_cp (mut self) -> Option < (char , StrCursor < 'a >) > { let cp = self . cp_after () ? ; self . seek_right (cp . len_utf8 ()) ; Some ((cp , self)) } fn slice_before (& self) -> & 'a str { & self . s [0 .. self . at] } pub (crate) fn slice_after (& self) -> & 'a str { & self . s [self . at ..] } pub (crate) fn slice_between (& self , until : StrCursor < 'a >) -> Option < & 'a str > { if ! str_eq_literal (self . s , until . s) { None } else { use std :: cmp :: { max , min } ; let beg = min (self . at , until . at) ; let end = max (self . at , until . at) ; Some (& self . s [beg .. end]) } } fn cp_after (& self) -> Option < char > { self . slice_after () . chars () . next () } fn try_seek_right_cp (& mut self) -> bool { match self . slice_after () . chars () . next () { Some (c) => { self . at += c . len_utf8 () ; true } None => false , } } fn seek_right (& mut self , bytes : usize) { self . at += bytes ; } }}}
mkitem!{mkimpl!{impl Copy for StrCursor < '_ > { }}}
mkitem!{mkimpl!{impl < 'a > Clone for StrCursor < 'a > { fn clone (& self) -> StrCursor < 'a > { * self } }}}
mkitem!{mkimpl!{impl std :: fmt :: Debug for StrCursor < '_ > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (fmt , "StrCursor({:?} | {:?})" , self . slice_before () , self . slice_after ()) } }}}

macro_rules! str_eq_literal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function str_eq_literal in module {}", module_path!());
    };
}

mkfn!{
    str_eq_literal_introspect!();
    fn str_eq_literal (a : & str , b : & str) -> bool { a . as_bytes () . as_ptr () == b . as_bytes () . as_ptr () && a . len () == b . len () }
} 
            }}