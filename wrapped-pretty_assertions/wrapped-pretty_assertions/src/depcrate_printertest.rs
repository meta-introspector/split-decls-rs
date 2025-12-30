// Generated macro for test (module)
macro_rules! Depcrate_printertest {
() => {
// Module: crate::printer
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [cfg (feature = "alloc")] use alloc :: string :: String ; const RED_LIGHT : & str = "\u{1b}[31m" ; const GREEN_LIGHT : & str = "\u{1b}[32m" ; const RED_HEAVY : & str = "\u{1b}[1;48;5;52;31m" ; const GREEN_HEAVY : & str = "\u{1b}[1;48;5;22;32m" ; const RESET : & str = "\u{1b}[0m" ; # [doc = " Given that both of our diff printing functions have the same"] # [doc = " type signature, we can reuse the same test code for them."] # [doc = ""] # [doc = " This could probably be nicer with traits!"] fn check_printer < TPrint > (printer : TPrint , left : & str , right : & str , expected : & str) where TPrint : Fn (& mut String , & str , & str) -> fmt :: Result , { let mut actual = String :: new () ; printer (& mut actual , left , right) . expect ("printer function failed") ; # [cfg (feature = "std")] println ! ("## left ##\n\
             {}\n\
             ## right ##\n\
             {}\n\
             ## actual diff ##\n\
             {}\n\
             ## expected diff ##\n\
             {}" , left , right , actual , expected) ; assert_eq ! (actual , expected) ; } # [test] fn write_inline_diff_empty () { let left = "" ; let right = "" ; let expected = format ! ("{red_light}<{reset}\n\
             {green_light}>{reset}\n" , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_inline_diff , left , right , & expected) ; } # [test] fn write_inline_diff_added () { let left = "" ; let right = "polymerase" ; let expected = format ! ("{red_light}<{reset}\n\
             {green_light}>{reset}{green_heavy}polymerase{reset}\n" , red_light = RED_LIGHT , green_light = GREEN_LIGHT , green_heavy = GREEN_HEAVY , reset = RESET ,) ; check_printer (write_inline_diff , left , right , & expected) ; } # [test] fn write_inline_diff_removed () { let left = "polyacrylamide" ; let right = "" ; let expected = format ! ("{red_light}<{reset}{red_heavy}polyacrylamide{reset}\n\
             {green_light}>{reset}\n" , red_light = RED_LIGHT , green_light = GREEN_LIGHT , red_heavy = RED_HEAVY , reset = RESET ,) ; check_printer (write_inline_diff , left , right , & expected) ; } # [test] fn write_inline_diff_changed () { let left = "polymerase" ; let right = "polyacrylamide" ; let expected = format ! ("{red_light}<poly{reset}{red_heavy}me{reset}{red_light}ra{reset}{red_heavy}s{reset}{red_light}e{reset}\n\
             {green_light}>poly{reset}{green_heavy}ac{reset}{green_light}r{reset}{green_heavy}yl{reset}{green_light}a{reset}{green_heavy}mid{reset}{green_light}e{reset}\n" , red_light = RED_LIGHT , green_light = GREEN_LIGHT , red_heavy = RED_HEAVY , green_heavy = GREEN_HEAVY , reset = RESET ,) ; check_printer (write_inline_diff , left , right , & expected) ; } # [doc = " If one of our strings is empty, it should not be shown at all in the output."] # [test] fn write_lines_empty_string () { let left = "" ; let right = "content" ; let expected = format ! ("{green_light}>content{reset}\n" , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [doc = " Realistic multiline struct diffing case."] # [test] fn write_lines_struct () { let left = r#"Some(
    Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok(
            "hey",
        ),
    },
)"# ; let right = r#"Some(
    Foo {
        lorem: "Hello Wrold!",
        ipsum: 42,
        dolor: Ok(
            "hey ho!",
        ),
    },
)"# ; let expected = format ! (r#" Some(
     Foo {{
{red_light}<        lorem: "Hello W{reset}{red_heavy}o{reset}{red_light}rld!",{reset}
{green_light}>        lorem: "Hello Wr{reset}{green_heavy}o{reset}{green_light}ld!",{reset}
         ipsum: 42,
         dolor: Ok(
{red_light}<            "hey",{reset}
{green_light}>            "hey{reset}{green_heavy} ho!{reset}{green_light}",{reset}
         ),
     }},
 )
"# , red_light = RED_LIGHT , red_heavy = RED_HEAVY , green_light = GREEN_LIGHT , green_heavy = GREEN_HEAVY , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [doc = " Relistic multiple line chunks"] # [doc = ""] # [doc = " We can't support realistic line diffing in large blocks"] # [doc = " (also, it's unclear how usefult this is)"] # [doc = ""] # [doc = " So if we have more than one line in a single removal chunk, disable inline diffing."] # [test] fn write_lines_multiline_block () { let left = r#"Proboscis
Cabbage"# ; let right = r#"Probed
Caravaggio"# ; let expected = format ! (r#"{red_light}<Proboscis{reset}
{red_light}<Cabbage{reset}
{green_light}>Probed{reset}
{green_light}>Caravaggio{reset}
"# , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [doc = " Single deletion line, multiple insertions - no inline diffing."] # [test] fn write_lines_multiline_insert () { let left = r#"Cabbage"# ; let right = r#"Probed
Caravaggio"# ; let expected = format ! (r#"{red_light}<Cabbage{reset}
{green_light}>Probed{reset}
{green_light}>Caravaggio{reset}
"# , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [doc = " Multiple deletion, single insertion - no inline diffing."] # [test] fn write_lines_multiline_delete () { let left = r#"Proboscis
Cabbage"# ; let right = r#"Probed"# ; let expected = format ! (r#"{red_light}<Proboscis{reset}
{red_light}<Cabbage{reset}
{green_light}>Probed{reset}
"# , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [doc = " Regression test for multiline highlighting issue"] # [test] fn write_lines_issue12 () { let left = r#"[
    0,
    0,
    0,
    128,
    10,
    191,
    5,
    64,
]"# ; let right = r#"[
    84,
    248,
    45,
    64,
]"# ; let expected = format ! (r#" [
{red_light}<    0,{reset}
{red_light}<    0,{reset}
{red_light}<    0,{reset}
{red_light}<    128,{reset}
{red_light}<    10,{reset}
{red_light}<    191,{reset}
{red_light}<    5,{reset}
{green_light}>    84,{reset}
{green_light}>    248,{reset}
{green_light}>    45,{reset}
     64,
 ]
"# , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } mod write_lines_edge_newlines { use super :: * ; # [test] fn both_trailing () { let left = "fan\n" ; let right = "mug\n" ; let expected = format ! (r#"{red_light}<{reset}{red_heavy}fan{reset}
{green_light}>{reset}{green_heavy}mug{reset}
 
"# , red_light = RED_LIGHT , red_heavy = RED_HEAVY , green_light = GREEN_LIGHT , green_heavy = GREEN_HEAVY , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [test] fn both_leading () { let left = "\nfan" ; let right = "\nmug" ; let expected = format ! (r#" 
{red_light}<{reset}{red_heavy}fan{reset}
{green_light}>{reset}{green_heavy}mug{reset}
"# , red_light = RED_LIGHT , red_heavy = RED_HEAVY , green_light = GREEN_LIGHT , green_heavy = GREEN_HEAVY , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [test] fn leading_added () { let left = "fan" ; let right = "\nmug" ; let expected = format ! (r#"{red_light}<fan{reset}
{green_light}>{reset}
{green_light}>mug{reset}
"# , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [test] fn leading_deleted () { let left = "\nfan" ; let right = "mug" ; let expected = format ! (r#"{red_light}<{reset}
{red_light}<fan{reset}
{green_light}>mug{reset}
"# , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [test] fn trailing_added () { let left = "fan" ; let right = "mug\n" ; let expected = format ! (r#"{red_light}<fan{reset}
{green_light}>mug{reset}
{green_light}>{reset}
"# , red_light = RED_LIGHT , green_light = GREEN_LIGHT , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } # [doc = " Regression test for double abort"] # [doc = ""] # [doc = " See: https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/96"] # [test] fn trailing_deleted () { let left = "fan\n" ; let right = "mug" ; let expected = format ! (r#"{red_light}<{reset}{red_heavy}fan{reset}
{green_light}>{reset}{green_heavy}mug{reset}
{red_light}<{reset}
"# , red_light = RED_LIGHT , red_heavy = RED_HEAVY , green_light = GREEN_LIGHT , green_heavy = GREEN_HEAVY , reset = RESET ,) ; check_printer (write_lines , left , right , & expected) ; } } }
};
}
