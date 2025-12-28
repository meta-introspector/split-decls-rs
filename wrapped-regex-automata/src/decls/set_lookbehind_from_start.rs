macro_rules! deps {
    () => {
        Look!();
        StateBuilderMatches!();
        Start!();
        NFA!();
    };
}

macro_rules! set_lookbehind_from_start {
    () => {
        deps!();
        # [doc = " Sets the appropriate look-behind assertions on the given state based on"] # [doc = " this starting configuration."] pub (crate) fn set_lookbehind_from_start (nfa : & thompson :: NFA , start : & Start , builder : & mut StateBuilderMatches ,) { let rev = nfa . is_reverse () ; let lineterm = nfa . look_matcher () . get_line_terminator () ; let lookset = nfa . look_set_any () ; match * start { Start :: NonWordByte => { if lookset . contains_word () { builder . set_look_have (| have | { have . insert (Look :: WordStartHalfAscii) . insert (Look :: WordStartHalfUnicode) }) ; } } Start :: WordByte => { if lookset . contains_word () { builder . set_is_from_word () ; } } Start :: Text => { if lookset . contains_anchor_haystack () { builder . set_look_have (| have | have . insert (Look :: Start)) ; } if lookset . contains_anchor_line () { builder . set_look_have (| have | { have . insert (Look :: StartLF) . insert (Look :: StartCRLF) }) ; } if lookset . contains_word () { builder . set_look_have (| have | { have . insert (Look :: WordStartHalfAscii) . insert (Look :: WordStartHalfUnicode) }) ; } } Start :: LineLF => { if rev { if lookset . contains_anchor_crlf () { builder . set_is_half_crlf () ; } if lookset . contains_anchor_line () { builder . set_look_have (| have | have . insert (Look :: StartLF)) ; } } else { if lookset . contains_anchor_line () { builder . set_look_have (| have | have . insert (Look :: StartCRLF)) ; } } if lookset . contains_anchor_line () && lineterm == b'\n' { builder . set_look_have (| have | have . insert (Look :: StartLF)) ; } if lookset . contains_word () { builder . set_look_have (| have | { have . insert (Look :: WordStartHalfAscii) . insert (Look :: WordStartHalfUnicode) }) ; } } Start :: LineCR => { if lookset . contains_anchor_crlf () { if rev { builder . set_look_have (| have | have . insert (Look :: StartCRLF)) ; } else { builder . set_is_half_crlf () ; } } if lookset . contains_anchor_line () && lineterm == b'\r' { builder . set_look_have (| have | have . insert (Look :: StartLF)) ; } if lookset . contains_word () { builder . set_look_have (| have | { have . insert (Look :: WordStartHalfAscii) . insert (Look :: WordStartHalfUnicode) }) ; } } Start :: CustomLineTerminator => { if lookset . contains_anchor_line () { builder . set_look_have (| have | have . insert (Look :: StartLF)) ; } if lookset . contains_word () { if utf8 :: is_word_byte (lineterm) { builder . set_is_from_word () ; } else { builder . set_look_have (| have | { have . insert (Look :: WordStartHalfAscii) . insert (Look :: WordStartHalfUnicode) }) ; } } } } }
    };
}

set_lookbehind_from_start!();