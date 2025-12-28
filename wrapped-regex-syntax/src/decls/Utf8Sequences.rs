macro_rules! deps {
    () => {
        ScalarRange!();
        Utf8Sequence!();
    };
}

macro_rules! Utf8Sequences {
    () => {
        deps!();
        # [doc = " An iterator over ranges of matching UTF-8 byte sequences."] # [doc = ""] # [doc = " The iteration represents an alternation of comprehensive byte sequences"] # [doc = " that match precisely the set of UTF-8 encoded scalar values."] # [doc = ""] # [doc = " A byte sequence corresponds to one of the scalar values in the range given"] # [doc = " if and only if it completely matches exactly one of the sequences of byte"] # [doc = " ranges produced by this iterator."] # [doc = ""] # [doc = " Each sequence of byte ranges matches a unique set of bytes. That is, no two"] # [doc = " sequences will match the same bytes."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to match an arbitrary byte sequence against a range of"] # [doc = " scalar values."] # [doc = ""] # [doc = " ```rust"] # [doc = " use regex_syntax::utf8::{Utf8Sequences, Utf8Sequence};"] # [doc = ""] # [doc = " fn matches(seqs: &[Utf8Sequence], bytes: &[u8]) -> bool {"] # [doc = "     for range in seqs {"] # [doc = "         if range.matches(bytes) {"] # [doc = "             return true;"] # [doc = "         }"] # [doc = "     }"] # [doc = "     false"] # [doc = " }"] # [doc = ""] # [doc = " // Test the basic multilingual plane."] # [doc = " let seqs: Vec<_> = Utf8Sequences::new('\\u{0}', '\\u{FFFF}').collect();"] # [doc = ""] # [doc = " // UTF-8 encoding of 'a'."] # [doc = " assert!(matches(&seqs, &[0x61]));"] # [doc = " // UTF-8 encoding of '☃' (`\\u{2603}`)."] # [doc = " assert!(matches(&seqs, &[0xE2, 0x98, 0x83]));"] # [doc = " // UTF-8 encoding of `\\u{10348}` (outside the BMP)."] # [doc = " assert!(!matches(&seqs, &[0xF0, 0x90, 0x8D, 0x88]));"] # [doc = " // Tries to match against a UTF-8 encoding of a surrogate codepoint,"] # [doc = " // which is invalid UTF-8, and therefore fails, despite the fact that"] # [doc = " // the corresponding codepoint (0xD800) falls in the range given."] # [doc = " assert!(!matches(&seqs, &[0xED, 0xA0, 0x80]));"] # [doc = " // And fails against plain old invalid UTF-8."] # [doc = " assert!(!matches(&seqs, &[0xFF, 0xFF]));"] # [doc = " ```"] # [doc = ""] # [doc = " If this example seems circuitous, that's because it is! It's meant to be"] # [doc = " illustrative. In practice, you could just try to decode your byte sequence"] # [doc = " and compare it with the scalar value range directly. However, this is not"] # [doc = " always possible (for example, in a byte based automaton)."] # [derive (Debug)] pub struct Utf8Sequences { range_stack : Vec < ScalarRange > , }
    };
}

Utf8Sequences!()