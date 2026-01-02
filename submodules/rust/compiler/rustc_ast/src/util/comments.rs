mkuse!{use rustc_span :: { BytePos , Symbol } ;}
mkuse!{use crate :: token :: CommentKind ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Debug)] pub enum CommentStyle { # [doc = " No code on either side of each line of the comment"] Isolated , # [doc = " Code exists to the left of the comment"] Trailing , # [doc = " Code before /* foo */ and after the comment"] Mixed , # [doc = " Just a manual blank line \"\\n\\n\", for layout"] BlankLine , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct Comment { pub style : CommentStyle , pub lines : Vec < String > , pub pos : BytePos , }}}

macro_rules! may_have_doc_links_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function may_have_doc_links in module {}", module_path!());
    };
}

mkfn!{
    may_have_doc_links_introspect!();
    # [doc = " A fast conservative estimate on whether the string can contain documentation links."] # [doc = " A pair of square brackets `[]` must exist in the string, but we only search for the"] # [doc = " opening bracket because brackets always go in pairs in practice."] # [inline] pub fn may_have_doc_links (s : & str) -> bool { s . contains ('[') }
}

macro_rules! beautify_doc_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function beautify_doc_string in module {}", module_path!());
    };
}

mkfn!{
    beautify_doc_string_introspect!();
    # [doc = " Makes a doc string more presentable to users."] # [doc = " Used by rustdoc and perhaps other tools, but not by rustc."] pub fn beautify_doc_string (data : Symbol , kind : CommentKind) -> Symbol { fn get_vertical_trim (lines : & [& str]) -> Option < (usize , usize) > { let mut i = 0 ; let mut j = lines . len () ; if lines . first () . is_some_and (| line | line . chars () . all (| c | c == '*')) { i += 1 ; } if j > i && ! lines [j - 1] . is_empty () && lines [j - 1] . chars () . all (| c | c == '*') { j -= 1 ; } if i != 0 || j != lines . len () { Some ((i , j)) } else { None } } fn get_horizontal_trim (lines : & [& str] , kind : CommentKind) -> Option < String > { let mut i = usize :: MAX ; let mut first = true ; let lines = match kind { CommentKind :: Block => { let mut i = lines . first () . map (| l | if l . trim_start () . starts_with ('*') { 0 } else { 1 }) . unwrap_or (0) ; let mut j = lines . len () ; while i < j && lines [i] . trim () . is_empty () { i += 1 ; } while j > i && lines [j - 1] . trim () . is_empty () { j -= 1 ; } & lines [i .. j] } CommentKind :: Line => lines , } ; for line in lines { for (j , c) in line . chars () . enumerate () { if j > i || ! "* \t" . contains (c) { return None ; } if c == '*' { if first { i = j ; first = false ; } else if i != j { return None ; } break ; } } if i >= line . len () { return None ; } } Some (lines . first () ? [.. i] . to_string ()) } let data_s = data . as_str () ; if data_s . contains ('\n') { let mut lines = data_s . lines () . collect :: < Vec < & str > > () ; let mut changes = false ; let lines = if let Some ((i , j)) = get_vertical_trim (& lines) { changes = true ; & mut lines [i .. j] } else { & mut lines } ; if let Some (horizontal) = get_horizontal_trim (lines , kind) { changes = true ; for line in lines . iter_mut () { if let Some (tmp) = line . strip_prefix (& horizontal) { * line = tmp ; if kind == CommentKind :: Block && (* line == "*" || line . starts_with ("* ") || line . starts_with ("**")) { * line = & line [1 ..] ; } } } } if changes { return Symbol :: intern (& lines . join ("\n")) ; } } data }
}