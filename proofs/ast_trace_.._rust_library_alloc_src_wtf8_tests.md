# AST Trace: ../rust/library/alloc/src/wtf8/tests.rs

Generated 57 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=code_point_from_u32 | COMPLEXITY=2 | LINES=11

```rust
use realalloc::string::ToString;

use super::*;

#[test]
fn code_point_from_u32() {
    assert!(CodePoint::from_u32(0).is_some());
    assert!(CodePoint::from_u32(0xD800).is_some());
    assert!(CodePoint::from_u32(0x10FFFF).is_some());
    assert!(CodePoint::from_u32(0x110000).is_none());
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=code_point_to_u32 | COMPLEXITY=3 | LINES=10

```rust
#[test]
fn code_point_to_u32() {
    fn c(value: u32) -> CodePoint {
        CodePoint::from_u32(value).unwrap()
    }
    assert_eq!(c(0).to_u32(), 0);
    assert_eq!(c(0xD800).to_u32(), 0xD800);
    assert_eq!(c(0x10FFFF).to_u32(), 0x10FFFF);
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=code_point_to_lead_surrogate | COMPLEXITY=4 | LINES=15

```rust
#[test]
fn code_point_to_lead_surrogate() {
    fn c(value: u32) -> CodePoint {
        CodePoint::from_u32(value).unwrap()
    }
    assert_eq!(c(0).to_lead_surrogate(), None);
    assert_eq!(c(0xE9).to_lead_surrogate(), None);
    assert_eq!(c(0xD800).to_lead_surrogate(), Some(0xD800));
    assert_eq!(c(0xDBFF).to_lead_surrogate(), Some(0xDBFF));
    assert_eq!(c(0xDC00).to_lead_surrogate(), None);
    assert_eq!(c(0xDFFF).to_lead_surrogate(), None);
    assert_eq!(c(0x1F4A9).to_lead_surrogate(), None);
    assert_eq!(c(0x10FFFF).to_lead_surrogate(), None);
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=code_point_to_trail_surrogate | COMPLEXITY=4 | LINES=15

```rust
#[test]
fn code_point_to_trail_surrogate() {
    fn c(value: u32) -> CodePoint {
        CodePoint::from_u32(value).unwrap()
    }
    assert_eq!(c(0).to_trail_surrogate(), None);
    assert_eq!(c(0xE9).to_trail_surrogate(), None);
    assert_eq!(c(0xD800).to_trail_surrogate(), None);
    assert_eq!(c(0xDBFF).to_trail_surrogate(), None);
    assert_eq!(c(0xDC00).to_trail_surrogate(), Some(0xDC00));
    assert_eq!(c(0xDFFF).to_trail_surrogate(), Some(0xDFFF));
    assert_eq!(c(0x1F4A9).to_trail_surrogate(), None);
    assert_eq!(c(0x10FFFF).to_trail_surrogate(), None);
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=code_point_from_char | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn code_point_from_char() {
    assert_eq!(CodePoint::from_char('a').to_u32(), 0x61);
    assert_eq!(CodePoint::from_char('💩').to_u32(), 0x1F4A9);
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=code_point_to_string | COMPLEXITY=4 | LINES=6

```rust
#[test]
fn code_point_to_string() {
    assert_eq!(format!("{:?}", CodePoint::from_char('a')), "U+0061");
    assert_eq!(format!("{:?}", CodePoint::from_char('💩')), "U+1F4A9");
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=code_point_to_char | COMPLEXITY=3 | LINES=10

```rust
#[test]
fn code_point_to_char() {
    fn c(value: u32) -> CodePoint {
        CodePoint::from_u32(value).unwrap()
    }
    assert_eq!(c(0x61).to_char(), Some('a'));
    assert_eq!(c(0x1F4A9).to_char(), Some('💩'));
    assert_eq!(c(0xD800).to_char(), None);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=code_point_to_char_lossy | COMPLEXITY=4 | LINES=10

```rust
#[test]
fn code_point_to_char_lossy() {
    fn c(value: u32) -> CodePoint {
        CodePoint::from_u32(value).unwrap()
    }
    assert_eq!(c(0x61).to_char_lossy(), 'a');
    assert_eq!(c(0x1F4A9).to_char_lossy(), '💩');
    assert_eq!(c(0xD800).to_char_lossy(), '\u{FFFD}');
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=wtf8buf_new | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn wtf8buf_new() {
    assert_eq!(Wtf8Buf::new().as_bytes(), b"");
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=wtf8buf_from_str | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn wtf8buf_from_str() {
    assert_eq!(Wtf8Buf::from_str("").as_bytes(), b"");
    assert_eq!(Wtf8Buf::from_str("aé 💩").as_bytes(), b"a\xC3\xA9 \xF0\x9F\x92\xA9");
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=wtf8buf_from_string | COMPLEXITY=2 | LINES=9

```rust
#[test]
fn wtf8buf_from_string() {
    assert_eq!(Wtf8Buf::from_string(String::from("")).as_bytes(), b"");
    assert_eq!(
        Wtf8Buf::from_string(String::from("aé 💩")).as_bytes(),
        b"a\xC3\xA9 \xF0\x9F\x92\xA9"
    );
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=wtf8buf_from_wide | COMPLEXITY=4 | LINES=31

```rust
#[test]
fn wtf8buf_from_wide() {
    let buf = Wtf8Buf::from_wide(&[]);
    assert_eq!(buf.as_bytes(), b"");
    assert!(buf.is_known_utf8);

    let buf = Wtf8Buf::from_wide(&[0x61, 0xE9, 0x20, 0xD83D, 0xDCA9]);
    assert_eq!(buf.as_bytes(), b"a\xC3\xA9 \xF0\x9F\x92\xA9");
    assert!(buf.is_known_utf8);

    let buf = Wtf8Buf::from_wide(&[0x61, 0xE9, 0x20, 0xD83D, 0xD83D, 0xDCA9]);
    assert_eq!(buf.as_bytes(), b"a\xC3\xA9 \xED\xA0\xBD\xF0\x9F\x92\xA9");
    assert!(!buf.is_known_utf8);

    let buf = Wtf8Buf::from_wide(&[0xD800]);
    assert_eq!(buf.as_bytes(), b"\xED\xA0\x80");
    assert!(!buf.is_known_utf8);

    let buf = Wtf8Buf::from_wide(&[0xDBFF]);
    assert_eq!(buf.as_bytes(), b"\xED\xAF\xBF");
    assert!(!buf.is_known_utf8);

    let buf = Wtf8Buf::from_wide(&[0xDC00]);
    assert_eq!(buf.as_bytes(), b"\xED\xB0\x80");
    assert!(!buf.is_known_utf8);

    let buf = Wtf8Buf::from_wide(&[0xDFFF]);
    assert_eq!(buf.as_bytes(), b"\xED\xBF\xBF");
    assert!(!buf.is_known_utf8);
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=wtf8buf_push_str | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn wtf8buf_push_str() {
    let mut string = Wtf8Buf::new();
    assert_eq!(string.as_bytes(), b"");
    assert!(string.is_known_utf8);

    string.push_str("aé 💩");
    assert_eq!(string.as_bytes(), b"a\xC3\xA9 \xF0\x9F\x92\xA9");
    assert!(string.is_known_utf8);
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=wtf8buf_push_char | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn wtf8buf_push_char() {
    let mut string = Wtf8Buf::from_str("aé ");
    assert_eq!(string.as_bytes(), b"a\xC3\xA9 ");
    assert!(string.is_known_utf8);

    string.push_char('💩');
    assert_eq!(string.as_bytes(), b"a\xC3\xA9 \xF0\x9F\x92\xA9");
    assert!(string.is_known_utf8);
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=wtf8buf_push | COMPLEXITY=6 | LINES=59

```rust
#[test]
fn wtf8buf_push() {
    let mut string = Wtf8Buf::from_str("aé ");
    assert_eq!(string.as_bytes(), b"a\xC3\xA9 ");
    assert!(string.is_known_utf8);

    string.push(CodePoint::from_char('💩'));
    assert_eq!(string.as_bytes(), b"a\xC3\xA9 \xF0\x9F\x92\xA9");
    assert!(string.is_known_utf8);

    fn c(value: u32) -> CodePoint {
        CodePoint::from_u32(value).unwrap()
    }

    let mut string = Wtf8Buf::new();
    string.push(c(0xD83D)); // lead
    assert!(!string.is_known_utf8);
    string.push(c(0xDCA9)); // trail
    assert_eq!(string.as_bytes(), b"\xF0\x9F\x92\xA9"); // Magic!

    let mut string = Wtf8Buf::new();
    string.push(c(0xD83D)); // lead
    assert!(!string.is_known_utf8);
    string.push(c(0x20)); // not surrogate
    string.push(c(0xDCA9)); // trail
    assert_eq!(string.as_bytes(), b"\xED\xA0\xBD \xED\xB2\xA9");

    let mut string = Wtf8Buf::new();
    string.push(c(0xD800)); // lead
    assert!(!string.is_known_utf8);
    string.push(c(0xDBFF)); // lead
    assert_eq!(string.as_bytes(), b"\xED\xA0\x80\xED\xAF\xBF");

    let mut string = Wtf8Buf::new();
    string.push(c(0xD800)); // lead
    assert!(!string.is_known_utf8);
    string.push(c(0xE000)); // not surrogate
    assert_eq!(string.as_bytes(), b"\xED\xA0\x80\xEE\x80\x80");

    let mut string = Wtf8Buf::new();
    string.push(c(0xD7FF)); // not surrogate
    assert!(string.is_known_utf8);
    string.push(c(0xDC00)); // trail
    assert!(!string.is_known_utf8);
    assert_eq!(string.as_bytes(), b"\xED\x9F\xBF\xED\xB0\x80");

    let mut string = Wtf8Buf::new();
    string.push(c(0x61)); // not surrogate, < 3 bytes
    assert!(string.is_known_utf8);
    string.push(c(0xDC00)); // trail
    assert!(!string.is_known_utf8);
    assert_eq!(string.as_bytes(), b"\x61\xED\xB0\x80");

    let mut string = Wtf8Buf::new();
    string.push(c(0xDC00)); // trail
    assert!(!string.is_known_utf8);
    assert_eq!(string.as_bytes(), b"\xED\xB0\x80");
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=wtf8buf_push_wtf8 | COMPLEXITY=12 | LINES=54

```rust
#[test]
fn wtf8buf_push_wtf8() {
    let mut string = Wtf8Buf::from_str("aé");
    assert_eq!(string.as_bytes(), b"a\xC3\xA9");
    string.push_wtf8(Wtf8::from_str(" 💩"));
    assert_eq!(string.as_bytes(), b"a\xC3\xA9 \xF0\x9F\x92\xA9");
    assert!(string.is_known_utf8);

    fn w(v: &[u8]) -> &Wtf8 {
        unsafe { Wtf8::from_bytes_unchecked(v) }
    }

    let mut string = Wtf8Buf::new();
    string.push_wtf8(w(b"\xED\xA0\xBD")); // lead
    string.push_wtf8(w(b"\xED\xB2\xA9")); // trail
    assert_eq!(string.as_bytes(), b"\xF0\x9F\x92\xA9"); // Magic!

    let mut string = Wtf8Buf::new();
    string.push_wtf8(w(b"\xED\xA0\xBD")); // lead
    string.push_wtf8(w(b" ")); // not surrogate
    string.push_wtf8(w(b"\xED\xB2\xA9")); // trail
    assert_eq!(string.as_bytes(), b"\xED\xA0\xBD \xED\xB2\xA9");
    assert!(!string.is_known_utf8);

    let mut string = Wtf8Buf::new();
    string.push_wtf8(w(b"\xED\xA0\x80")); // lead
    string.push_wtf8(w(b"\xED\xAF\xBF")); // lead
    assert_eq!(string.as_bytes(), b"\xED\xA0\x80\xED\xAF\xBF");
    assert!(!string.is_known_utf8);

    let mut string = Wtf8Buf::new();
    string.push_wtf8(w(b"\xED\xA0\x80")); // lead
    string.push_wtf8(w(b"\xEE\x80\x80")); // not surrogate
    assert_eq!(string.as_bytes(), b"\xED\xA0\x80\xEE\x80\x80");
    assert!(!string.is_known_utf8);

    let mut string = Wtf8Buf::new();
    string.push_wtf8(w(b"\xED\x9F\xBF")); // not surrogate
    string.push_wtf8(w(b"\xED\xB0\x80")); // trail
    assert_eq!(string.as_bytes(), b"\xED\x9F\xBF\xED\xB0\x80");
    assert!(!string.is_known_utf8);

    let mut string = Wtf8Buf::new();
    string.push_wtf8(w(b"a")); // not surrogate, < 3 bytes
    string.push_wtf8(w(b"\xED\xB0\x80")); // trail
    assert_eq!(string.as_bytes(), b"\x61\xED\xB0\x80");
    assert!(!string.is_known_utf8);

    let mut string = Wtf8Buf::new();
    string.push_wtf8(w(b"\xED\xB0\x80")); // trail
    assert_eq!(string.as_bytes(), b"\xED\xB0\x80");
    assert!(!string.is_known_utf8);
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=wtf8buf_truncate | COMPLEXITY=2 | LINES=18

```rust
#[test]
fn wtf8buf_truncate() {
    let mut string = Wtf8Buf::from_str("aé");
    assert!(string.is_known_utf8);

    string.truncate(3);
    assert_eq!(string.as_bytes(), b"a\xC3\xA9");
    assert!(string.is_known_utf8);

    string.truncate(1);
    assert_eq!(string.as_bytes(), b"a");
    assert!(string.is_known_utf8);

    string.truncate(0);
    assert_eq!(string.as_bytes(), b"");
    assert!(string.is_known_utf8);
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=wtf8buf_truncate_around_non_bmp | COMPLEXITY=2 | LINES=14

```rust
#[test]
fn wtf8buf_truncate_around_non_bmp() {
    let mut string = Wtf8Buf::from_str("💩");
    assert!(string.is_known_utf8);

    string.truncate(4);
    assert_eq!(string.as_bytes(), b"\xF0\x9F\x92\xA9");
    assert!(string.is_known_utf8);

    string.truncate(0);
    assert_eq!(string.as_bytes(), b"");
    assert!(string.is_known_utf8);
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=wtf8buf_truncate_fail_code_point_boundary | COMPLEXITY=2 | LINES=7

```rust
#[test]
#[should_panic]
fn wtf8buf_truncate_fail_code_point_boundary() {
    let mut string = Wtf8Buf::from_str("aé");
    string.truncate(2);
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=wtf8buf_truncate_fail_longer | COMPLEXITY=2 | LINES=7

```rust
#[test]
#[should_panic]
fn wtf8buf_truncate_fail_longer() {
    let mut string = Wtf8Buf::from_str("aé");
    string.truncate(4);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=wtf8buf_truncate_splitting_non_bmp3 | COMPLEXITY=2 | LINES=8

```rust
#[test]
#[should_panic]
fn wtf8buf_truncate_splitting_non_bmp3() {
    let mut string = Wtf8Buf::from_str("💩");
    assert!(string.is_known_utf8);
    string.truncate(3);
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=wtf8buf_truncate_splitting_non_bmp2 | COMPLEXITY=2 | LINES=8

```rust
#[test]
#[should_panic]
fn wtf8buf_truncate_splitting_non_bmp2() {
    let mut string = Wtf8Buf::from_str("💩");
    assert!(string.is_known_utf8);
    string.truncate(2);
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=wtf8buf_truncate_splitting_non_bmp1 | COMPLEXITY=2 | LINES=8

```rust
#[test]
#[should_panic]
fn wtf8buf_truncate_splitting_non_bmp1() {
    let mut string = Wtf8Buf::from_str("💩");
    assert!(string.is_known_utf8);
    string.truncate(1);
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=wtf8buf_into_string | COMPLEXITY=2 | LINES=10

```rust
#[test]
fn wtf8buf_into_string() {
    let mut string = Wtf8Buf::from_str("aé 💩");
    assert!(string.is_known_utf8);
    assert_eq!(string.clone().into_string(), Ok(String::from("aé 💩")));
    string.push(CodePoint::from_u32(0xD800).unwrap());
    assert!(!string.is_known_utf8);
    assert_eq!(string.clone().into_string(), Err(string));
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=wtf8buf_into_string_lossy | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn wtf8buf_into_string_lossy() {
    let mut string = Wtf8Buf::from_str("aé 💩");
    assert_eq!(string.clone().into_string_lossy(), String::from("aé 💩"));
    string.push(CodePoint::from_u32(0xD800).unwrap());
    assert_eq!(string.clone().into_string_lossy(), String::from("aé 💩�"));
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=wtf8buf_from_iterator | COMPLEXITY=12 | LINES=34

```rust
#[test]
fn wtf8buf_from_iterator() {
    fn f(values: &[u32]) -> Wtf8Buf {
        values.iter().map(|&c| CodePoint::from_u32(c).unwrap()).collect::<Wtf8Buf>()
    }
    assert_eq!(
        f(&[0x61, 0xE9, 0x20, 0x1F4A9]),
        Wtf8Buf { bytes: b"a\xC3\xA9 \xF0\x9F\x92\xA9".to_vec(), is_known_utf8: true }
    );

    assert_eq!(f(&[0xD83D, 0xDCA9]).as_bytes(), b"\xF0\x9F\x92\xA9"); // Magic!
    assert_eq!(
        f(&[0xD83D, 0x20, 0xDCA9]),
        Wtf8Buf { bytes: b"\xED\xA0\xBD \xED\xB2\xA9".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        f(&[0xD800, 0xDBFF]),
        Wtf8Buf { bytes: b"\xED\xA0\x80\xED\xAF\xBF".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        f(&[0xD800, 0xE000]),
        Wtf8Buf { bytes: b"\xED\xA0\x80\xEE\x80\x80".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        f(&[0xD7FF, 0xDC00]),
        Wtf8Buf { bytes: b"\xED\x9F\xBF\xED\xB0\x80".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        f(&[0x61, 0xDC00]),
        Wtf8Buf { bytes: b"\x61\xED\xB0\x80".to_vec(), is_known_utf8: false }
    );
    assert_eq!(f(&[0xDC00]), Wtf8Buf { bytes: b"\xED\xB0\x80".to_vec(), is_known_utf8: false });
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=wtf8buf_extend | COMPLEXITY=13 | LINES=43

```rust
#[test]
fn wtf8buf_extend() {
    fn e(initial: &[u32], extended: &[u32]) -> Wtf8Buf {
        fn c(value: &u32) -> CodePoint {
            CodePoint::from_u32(*value).unwrap()
        }
        let mut string = initial.iter().map(c).collect::<Wtf8Buf>();
        string.extend(extended.iter().map(c));
        string
    }

    assert_eq!(
        e(&[0x61, 0xE9], &[0x20, 0x1F4A9]),
        Wtf8Buf { bytes: b"a\xC3\xA9 \xF0\x9F\x92\xA9".to_vec(), is_known_utf8: true }
    );

    assert_eq!(e(&[0xD83D], &[0xDCA9]).as_bytes(), b"\xF0\x9F\x92\xA9"); // Magic!
    assert_eq!(
        e(&[0xD83D, 0x20], &[0xDCA9]),
        Wtf8Buf { bytes: b"\xED\xA0\xBD \xED\xB2\xA9".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        e(&[0xD800], &[0xDBFF]),
        Wtf8Buf { bytes: b"\xED\xA0\x80\xED\xAF\xBF".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        e(&[0xD800], &[0xE000]),
        Wtf8Buf { bytes: b"\xED\xA0\x80\xEE\x80\x80".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        e(&[0xD7FF], &[0xDC00]),
        Wtf8Buf { bytes: b"\xED\x9F\xBF\xED\xB0\x80".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        e(&[0x61], &[0xDC00]),
        Wtf8Buf { bytes: b"\x61\xED\xB0\x80".to_vec(), is_known_utf8: false }
    );
    assert_eq!(
        e(&[], &[0xDC00]),
        Wtf8Buf { bytes: b"\xED\xB0\x80".to_vec(), is_known_utf8: false }
    );
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=wtf8buf_show | COMPLEXITY=7 | LINES=7

```rust
#[test]
fn wtf8buf_show() {
    let mut string = Wtf8Buf::from_str("a\té \u{7f}💩\r");
    string.push(CodePoint::from_u32(0xD800).unwrap());
    assert_eq!(format!("{string:?}"), "\"a\\té \\u{7f}\u{1f4a9}\\r\\u{d800}\"");
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=wtf8buf_as_slice | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn wtf8buf_as_slice() {
    assert_eq!(Wtf8Buf::from_str("aé").as_slice(), Wtf8::from_str("aé"));
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=wtf8buf_show_str | COMPLEXITY=4 | LINES=7

```rust
#[test]
fn wtf8buf_show_str() {
    let text = "a\té 💩\r";
    let string = Wtf8Buf::from_str(text);
    assert_eq!(format!("{text:?}"), format!("{string:?}"));
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=wtf8_from_str | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn wtf8_from_str() {
    assert_eq!(&Wtf8::from_str("").as_bytes(), b"");
    assert_eq!(&Wtf8::from_str("aé 💩").as_bytes(), b"a\xC3\xA9 \xF0\x9F\x92\xA9");
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=wtf8_len | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn wtf8_len() {
    assert_eq!(Wtf8::from_str("").len(), 0);
    assert_eq!(Wtf8::from_str("aé 💩").len(), 8);
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=wtf8_slice | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn wtf8_slice() {
    assert_eq!(&Wtf8::from_str("aé 💩")[1..4].as_bytes(), b"\xC3\xA9 ");
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=wtf8_slice_not_code_point_boundary | COMPLEXITY=2 | LINES=6

```rust
#[test]
#[should_panic]
fn wtf8_slice_not_code_point_boundary() {
    let _ = &Wtf8::from_str("aé 💩")[2..4];
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=wtf8_slice_from | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn wtf8_slice_from() {
    assert_eq!(&Wtf8::from_str("aé 💩")[1..].as_bytes(), b"\xC3\xA9 \xF0\x9F\x92\xA9");
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=wtf8_slice_from_not_code_point_boundary | COMPLEXITY=2 | LINES=6

```rust
#[test]
#[should_panic]
fn wtf8_slice_from_not_code_point_boundary() {
    let _ = &Wtf8::from_str("aé 💩")[2..];
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=wtf8_slice_to | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn wtf8_slice_to() {
    assert_eq!(&Wtf8::from_str("aé 💩")[..4].as_bytes(), b"a\xC3\xA9 ");
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=wtf8_slice_to_not_code_point_boundary | COMPLEXITY=2 | LINES=6

```rust
#[test]
#[should_panic]
fn wtf8_slice_to_not_code_point_boundary() {
    let _ = &Wtf8::from_str("aé 💩")[5..];
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=wtf8_ascii_byte_at | COMPLEXITY=2 | LINES=10

```rust
#[test]
fn wtf8_ascii_byte_at() {
    let slice = Wtf8::from_str("aé 💩");
    assert_eq!(slice.ascii_byte_at(0), b'a');
    assert_eq!(slice.ascii_byte_at(1), b'\xFF');
    assert_eq!(slice.ascii_byte_at(2), b'\xFF');
    assert_eq!(slice.ascii_byte_at(3), b' ');
    assert_eq!(slice.ascii_byte_at(4), b'\xFF');
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=wtf8_code_points | COMPLEXITY=5 | LINES=16

```rust
#[test]
fn wtf8_code_points() {
    fn c(value: u32) -> CodePoint {
        CodePoint::from_u32(value).unwrap()
    }
    fn cp(string: &Wtf8Buf) -> Vec<Option<char>> {
        string.code_points().map(|c| c.to_char()).collect::<Vec<_>>()
    }
    let mut string = Wtf8Buf::from_str("é ");
    assert_eq!(cp(&string), [Some('é'), Some(' ')]);
    string.push(c(0xD83D));
    assert_eq!(cp(&string), [Some('é'), Some(' '), None]);
    string.push(c(0xDCA9));
    assert_eq!(cp(&string), [Some('é'), Some(' '), Some('💩')]);
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=wtf8_as_str | COMPLEXITY=2 | LINES=9

```rust
#[test]
fn wtf8_as_str() {
    assert_eq!(Wtf8::from_str("").as_str(), Ok(""));
    assert_eq!(Wtf8::from_str("aé 💩").as_str(), Ok("aé 💩"));
    let mut string = Wtf8Buf::new();
    string.push(CodePoint::from_u32(0xD800).unwrap());
    assert!(string.as_str().is_err());
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=wtf8_to_string_lossy | COMPLEXITY=2 | LINES=10

```rust
#[test]
fn wtf8_to_string_lossy() {
    assert_eq!(to_string_lossy(Wtf8::from_str("")), Cow::Borrowed(""));
    assert_eq!(to_string_lossy(Wtf8::from_str("aé 💩")), Cow::Borrowed("aé 💩"));
    let mut string = Wtf8Buf::from_str("aé 💩");
    string.push(CodePoint::from_u32(0xD800).unwrap());
    let expected: Cow<'_, str> = Cow::Owned(String::from("aé 💩�"));
    assert_eq!(to_string_lossy(&string), expected);
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=wtf8_display | COMPLEXITY=8 | LINES=14

```rust
#[test]
fn wtf8_display() {
    fn d(b: &[u8]) -> String {
        (&unsafe { Wtf8::from_bytes_unchecked(b) }).to_string()
    }

    assert_eq!("", d("".as_bytes()));
    assert_eq!("aé 💩", d("aé 💩".as_bytes()));

    let mut string = Wtf8Buf::from_str("aé 💩");
    string.push(CodePoint::from_u32(0xD800).unwrap());
    assert_eq!("aé 💩�", d(string.as_ref()));
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=wtf8_encode_wide | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn wtf8_encode_wide() {
    let mut string = Wtf8Buf::from_str("aé ");
    string.push(CodePoint::from_u32(0xD83D).unwrap());
    string.push_char('💩');
    assert_eq!(
        string.encode_wide().collect::<Vec<_>>(),
        vec![0x61, 0xE9, 0x20, 0xD83D, 0xD83D, 0xDCA9]
    );
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=wtf8_encode_wide_size_hint | COMPLEXITY=3 | LINES=12

```rust
#[test]
fn wtf8_encode_wide_size_hint() {
    let string = Wtf8Buf::from_str("\u{12345}");
    let mut iter = string.encode_wide();
    assert_eq!((1, Some(8)), iter.size_hint());
    iter.next().unwrap();
    assert_eq!((1, Some(1)), iter.size_hint());
    iter.next().unwrap();
    assert_eq!((0, Some(0)), iter.size_hint());
    assert!(iter.next().is_none());
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=wtf8_clone_into | COMPLEXITY=8 | LINES=25

```rust
#[test]
fn wtf8_clone_into() {
    let mut string = Wtf8Buf::new();
    clone_into(Wtf8::from_str("green"), &mut string);
    assert_eq!(string.as_bytes(), b"green");

    let mut string = Wtf8Buf::from_str("green");
    clone_into(Wtf8::from_str(""), &mut string);
    assert_eq!(string.as_bytes(), b"");

    let mut string = Wtf8Buf::from_str("red");
    clone_into(Wtf8::from_str("green"), &mut string);
    assert_eq!(string.as_bytes(), b"green");

    let mut string = Wtf8Buf::from_str("green");
    clone_into(Wtf8::from_str("red"), &mut string);
    assert_eq!(string.as_bytes(), b"red");

    let mut string = Wtf8Buf::from_str("green");
    assert!(string.is_known_utf8);
    clone_into(unsafe { Wtf8::from_bytes_unchecked(b"\xED\xA0\x80") }, &mut string);
    assert_eq!(string.as_bytes(), b"\xED\xA0\x80");
    assert!(!string.is_known_utf8);
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=wtf8_make_ascii_lowercase | COMPLEXITY=8 | LINES=16

```rust
#[test]
fn wtf8_make_ascii_lowercase() {
    let mut lowercase = Wtf8Buf::from_str("");
    lowercase.make_ascii_lowercase();
    assert_eq!(lowercase.as_bytes(), b"");

    let mut lowercase = Wtf8Buf::from_str("GrEeN gRaPeS! 🍇");
    lowercase.make_ascii_lowercase();
    assert_eq!(lowercase.as_bytes(), b"green grapes! \xf0\x9f\x8d\x87");

    let mut lowercase = to_owned(unsafe { Wtf8::from_bytes_unchecked(b"\xED\xA0\x80") });
    lowercase.make_ascii_lowercase();
    assert_eq!(lowercase.as_bytes(), b"\xED\xA0\x80");
    assert!(!lowercase.is_known_utf8);
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=wtf8_make_ascii_uppercase | COMPLEXITY=8 | LINES=16

```rust
#[test]
fn wtf8_make_ascii_uppercase() {
    let mut uppercase = Wtf8Buf::from_str("");
    uppercase.make_ascii_uppercase();
    assert_eq!(uppercase.as_bytes(), b"");

    let mut uppercase = Wtf8Buf::from_str("GrEeN gRaPeS! 🍇");
    uppercase.make_ascii_uppercase();
    assert_eq!(uppercase.as_bytes(), b"GREEN GRAPES! \xf0\x9f\x8d\x87");

    let mut uppercase = to_owned(unsafe { Wtf8::from_bytes_unchecked(b"\xED\xA0\x80") });
    uppercase.make_ascii_uppercase();
    assert_eq!(uppercase.as_bytes(), b"\xED\xA0\x80");
    assert!(!uppercase.is_known_utf8);
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=wtf8_to_owned | COMPLEXITY=7 | LINES=7

```rust
#[test]
fn wtf8_to_owned() {
    let string = to_owned(unsafe { Wtf8::from_bytes_unchecked(b"\xED\xA0\x80") });
    assert_eq!(string.as_bytes(), b"\xED\xA0\x80");
    assert!(!string.is_known_utf8);
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=wtf8_valid_utf8_boundaries | COMPLEXITY=5 | LINES=31

```rust
#[test]
fn wtf8_valid_utf8_boundaries() {
    let mut string = Wtf8Buf::from_str("aé 💩");
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.check_utf8_boundary(0);
    string.check_utf8_boundary(1);
    string.check_utf8_boundary(3);
    string.check_utf8_boundary(4);
    string.check_utf8_boundary(8);
    string.check_utf8_boundary(14);
    assert_eq!(string.len(), 14);

    string.push_char('a');
    string.check_utf8_boundary(14);
    string.check_utf8_boundary(15);

    let mut string = Wtf8Buf::from_str("a");
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.check_utf8_boundary(1);

    let mut string = Wtf8Buf::from_str("\u{D7FF}");
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.check_utf8_boundary(3);

    let mut string = Wtf8Buf::new();
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.push_char('\u{D7FF}');
    string.check_utf8_boundary(3);
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=wtf8_utf8_boundary_out_of_bounds | COMPLEXITY=2 | LINES=7

```rust
#[test]
#[should_panic(expected = "byte index 4 is out of bounds")]
fn wtf8_utf8_boundary_out_of_bounds() {
    let string = Wtf8::from_str("aé");
    string.check_utf8_boundary(4);
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=wtf8_utf8_boundary_inside_codepoint | COMPLEXITY=2 | LINES=7

```rust
#[test]
#[should_panic(expected = "byte index 1 is not a codepoint boundary")]
fn wtf8_utf8_boundary_inside_codepoint() {
    let string = Wtf8::from_str("é");
    string.check_utf8_boundary(1);
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=wtf8_utf8_boundary_inside_surrogate | COMPLEXITY=2 | LINES=8

```rust
#[test]
#[should_panic(expected = "byte index 1 is not a codepoint boundary")]
fn wtf8_utf8_boundary_inside_surrogate() {
    let mut string = Wtf8Buf::new();
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.check_utf8_boundary(1);
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=wtf8_utf8_boundary_between_surrogates | COMPLEXITY=2 | LINES=9

```rust
#[test]
#[should_panic(expected = "byte index 3 lies between surrogate codepoints")]
fn wtf8_utf8_boundary_between_surrogates() {
    let mut string = Wtf8Buf::new();
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.push(CodePoint::from_u32(0xD800).unwrap());
    string.check_utf8_boundary(3);
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=wobbled_wtf8_plus_bytes_isnt_utf8 | COMPLEXITY=12 | LINES=10

```rust
#[test]
fn wobbled_wtf8_plus_bytes_isnt_utf8() {
    let mut string: Wtf8Buf = to_owned(unsafe { Wtf8::from_bytes_unchecked(b"\xED\xA0\x80") });
    assert!(!string.is_known_utf8);
    unsafe {
        string.extend_from_slice_unchecked(b"some utf-8");
    }
    assert!(!string.is_known_utf8);
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=wobbled_wtf8_plus_str_isnt_utf8 | COMPLEXITY=7 | LINES=8

```rust
#[test]
fn wobbled_wtf8_plus_str_isnt_utf8() {
    let mut string: Wtf8Buf = to_owned(unsafe { Wtf8::from_bytes_unchecked(b"\xED\xA0\x80") });
    assert!(!string.is_known_utf8);
    string.push_str("some utf-8");
    assert!(!string.is_known_utf8);
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=FUNCTION | NAME=unwobbly_wtf8_plus_utf8_is_utf8 | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn unwobbly_wtf8_plus_utf8_is_utf8() {
    let mut string: Wtf8Buf = Wtf8Buf::from_str("hello world");
    assert!(string.is_known_utf8);
    string.push_str("some utf-8");
    assert!(string.is_known_utf8);
}
```

---
*Generated by AST tracing system*
