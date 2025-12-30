// Generated macro for process_author_str (function)
macro_rules! Depcrate_itemprocess_author_str {
() => {
// Module: crate::item
// Provides: {"process_author_str"}
// Dependencies: {}
# [doc = " replace all `:` with `, ` when not inside the `<>`"] # [doc = ""] # [doc = " `\"author1:author2:author3\" => \"author1, author2, author3\"`"] # [doc = " `\"author1 <http://website1.com>:author2\" => \"author1 <http://website1.com>, author2\"`"] fn process_author_str (author : & str) -> String { let mut res = String :: with_capacity (author . len ()) ; let mut inside_angle_braces = 0usize ; for ch in author . chars () { if inside_angle_braces > 0 && ch == '>' { inside_angle_braces -= 1 ; res . push (ch) ; } else if ch == '<' { inside_angle_braces += 1 ; res . push (ch) ; } else if inside_angle_braces == 0 && ch == ':' { res . push_str (", ") ; } else { res . push (ch) ; } } res }
};
}
