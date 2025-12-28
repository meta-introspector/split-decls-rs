macro_rules! deps {
    () => {
        Bytes!();
        ClassBytes!();
        ClassUnicode!();
    };
}

macro_rules! Class {
    () => {
        deps!();
        # [doc = " The high-level intermediate representation of a character class."] # [doc = ""] # [doc = " A character class corresponds to a set of characters. A character is either"] # [doc = " defined by a Unicode scalar value or a byte."] # [doc = ""] # [doc = " A character class, regardless of its character type, is represented by a"] # [doc = " sequence of non-overlapping non-adjacent ranges of characters."] # [doc = ""] # [doc = " There are no guarantees about which class variant is used. Generally"] # [doc = " speaking, the Unicode variant is used whenever a class needs to contain"] # [doc = " non-ASCII Unicode scalar values. But the Unicode variant can be used even"] # [doc = " when Unicode mode is disabled. For example, at the time of writing, the"] # [doc = " regex `(?-u:a|\\xc2\\xa0)` will compile down to HIR for the Unicode class"] # [doc = " `[a\\u00A0]` due to optimizations."] # [doc = ""] # [doc = " Note that `Bytes` variant may be produced even when it exclusively matches"] # [doc = " valid UTF-8. This is because a `Bytes` variant represents an intention by"] # [doc = " the author of the regular expression to disable Unicode mode, which in turn"] # [doc = " impacts the semantics of case insensitive matching. For example, `(?i)k`"] # [doc = " and `(?i-u)k` will not match the same set of strings."] # [derive (Clone , Eq , PartialEq)] pub enum Class { # [doc = " A set of characters represented by Unicode scalar values."] Unicode (ClassUnicode) , # [doc = " A set of characters represented by arbitrary bytes (one byte per"] # [doc = " character)."] Bytes (ClassBytes) , }
    };
}

Class!();