// Generated macro for tests (module)
macro_rules! Depcrate_macostests {
() => {
// Module: crate::macos
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: macos ; use crate :: { IterationControl , Segment , SharedLibrary } ; # [test] fn have_libdyld () { let mut found_dyld = false ; macos :: SharedLibrary :: each (| shlib | { found_dyld |= shlib . name . to_bytes () . split (| c | * c == b'.' || * c == b'/') . any (| s | s == b"libdyld") ; }) ; assert ! (found_dyld) ; } # [test] fn can_break () { let mut first_count = 0 ; macos :: SharedLibrary :: each (| _ | { first_count += 1 ; }) ; assert ! (first_count > 2) ; let mut second_count = 0 ; macos :: SharedLibrary :: each (| _ | { second_count += 1 ; if second_count == first_count - 1 { IterationControl :: Break } else { IterationControl :: Continue } }) ; assert_eq ! (second_count , first_count - 1) ; } # [test] fn get_name () { macos :: SharedLibrary :: each (| shlib | { let _ = shlib . name () ; }) ; } # [test] fn get_id () { macos :: SharedLibrary :: each (| shlib | { assert ! (shlib . id () . is_some ()) ; }) ; } # [test] fn have_text_or_pagezero () { macos :: SharedLibrary :: each (| shlib | { println ! ("shlib = {:?}" , shlib . name ()) ; let mut found_text_or_pagezero = false ; for seg in shlib . segments () { println ! ("    segment = {:?}" , seg . name ()) ; found_text_or_pagezero |= seg . name () == "__TEXT" ; found_text_or_pagezero |= seg . name () == "__PAGEZERO" ; } assert ! (found_text_or_pagezero) ; }) ; } }
};
}
