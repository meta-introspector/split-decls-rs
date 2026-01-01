# AST Trace: ../rust/library/std/src/io/util/tests.rs

Generated 11 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use crate::fmt;
use crate::io::prelude::*;
use crate::io::{
    BorrowedBuf, Empty, ErrorKind, IoSlice, IoSliceMut, Repeat, SeekFrom, Sink, empty, repeat, sink,
};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=ErrorDisplay; | COMPLEXITY=5 | LINES=9

```rust
use crate::mem::MaybeUninit;

struct ErrorDisplay;

impl fmt::Display for ErrorDisplay {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Err(fmt::Error)
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=PanicDisplay; | COMPLEXITY=5 | LINES=8

```rust
struct PanicDisplay;

impl fmt::Display for PanicDisplay {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!()
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=test_sinking | COMPLEXITY=5 | LINES=22

```rust
#[track_caller]
fn test_sinking<W: Write>(mut w: W) {
    assert_eq!(w.write(&[]).unwrap(), 0);
    assert_eq!(w.write(&[0]).unwrap(), 1);
    assert_eq!(w.write(&[0; 1024]).unwrap(), 1024);
    w.write_all(&[]).unwrap();
    w.write_all(&[0]).unwrap();
    w.write_all(&[0; 1024]).unwrap();
    let mut bufs =
        [IoSlice::new(&[]), IoSlice::new(&[0]), IoSlice::new(&[0; 1024]), IoSlice::new(&[])];
    assert!(w.is_write_vectored());
    assert_eq!(w.write_vectored(&[]).unwrap(), 0);
    assert_eq!(w.write_vectored(&bufs).unwrap(), 1025);
    w.write_all_vectored(&mut []).unwrap();
    w.write_all_vectored(&mut bufs).unwrap();
    assert!(w.flush().is_ok());
    assert_eq!(w.by_ref().write(&[0; 1024]).unwrap(), 1024);
    // Ignores fmt arguments
    w.write_fmt(format_args!("{}", ErrorDisplay)).unwrap();
    w.write_fmt(format_args!("{}", PanicDisplay)).unwrap();
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=sink_sinks | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn sink_sinks() {
    test_sinking(sink());
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=empty_reads | COMPLEXITY=8 | LINES=89

```rust
#[test]
fn empty_reads() {
    let mut e = empty();
    assert_eq!(e.read(&mut []).unwrap(), 0);
    assert_eq!(e.read(&mut [0]).unwrap(), 0);
    assert_eq!(e.read(&mut [0; 1024]).unwrap(), 0);
    assert_eq!(Read::by_ref(&mut e).read(&mut [0; 1024]).unwrap(), 0);

    e.read_exact(&mut []).unwrap();
    assert_eq!(e.read_exact(&mut [0]).unwrap_err().kind(), ErrorKind::UnexpectedEof);
    assert_eq!(e.read_exact(&mut [0; 1024]).unwrap_err().kind(), ErrorKind::UnexpectedEof);

    assert!(!e.is_read_vectored());
    assert_eq!(e.read_vectored(&mut []).unwrap(), 0);
    let (mut buf1, mut buf1024) = ([0], [0; 1024]);
    let bufs = &mut [
        IoSliceMut::new(&mut []),
        IoSliceMut::new(&mut buf1),
        IoSliceMut::new(&mut buf1024),
        IoSliceMut::new(&mut []),
    ];
    assert_eq!(e.read_vectored(bufs).unwrap(), 0);

    let buf: &mut [MaybeUninit<_>] = &mut [];
    let mut buf: BorrowedBuf<'_> = buf.into();
    e.read_buf(buf.unfilled()).unwrap();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let buf: &mut [_] = &mut [MaybeUninit::uninit()];
    let mut buf: BorrowedBuf<'_> = buf.into();
    e.read_buf(buf.unfilled()).unwrap();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let buf: &mut [_] = &mut [MaybeUninit::uninit(); 1024];
    let mut buf: BorrowedBuf<'_> = buf.into();
    e.read_buf(buf.unfilled()).unwrap();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let buf: &mut [_] = &mut [MaybeUninit::uninit(); 1024];
    let mut buf: BorrowedBuf<'_> = buf.into();
    Read::by_ref(&mut e).read_buf(buf.unfilled()).unwrap();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let buf: &mut [MaybeUninit<_>] = &mut [];
    let mut buf: BorrowedBuf<'_> = buf.into();
    e.read_buf_exact(buf.unfilled()).unwrap();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let buf: &mut [_] = &mut [MaybeUninit::uninit()];
    let mut buf: BorrowedBuf<'_> = buf.into();
    assert_eq!(e.read_buf_exact(buf.unfilled()).unwrap_err().kind(), ErrorKind::UnexpectedEof);
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let buf: &mut [_] = &mut [MaybeUninit::uninit(); 1024];
    let mut buf: BorrowedBuf<'_> = buf.into();
    assert_eq!(e.read_buf_exact(buf.unfilled()).unwrap_err().kind(), ErrorKind::UnexpectedEof);
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let buf: &mut [_] = &mut [MaybeUninit::uninit(); 1024];
    let mut buf: BorrowedBuf<'_> = buf.into();
    assert_eq!(
        Read::by_ref(&mut e).read_buf_exact(buf.unfilled()).unwrap_err().kind(),
        ErrorKind::UnexpectedEof,
    );
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.init_len(), 0);

    let mut buf = Vec::new();
    assert_eq!(e.read_to_end(&mut buf).unwrap(), 0);
    assert_eq!(buf, vec![]);
    let mut buf = vec![1, 2, 3];
    assert_eq!(e.read_to_end(&mut buf).unwrap(), 0);
    assert_eq!(buf, vec![1, 2, 3]);

    let mut buf = String::new();
    assert_eq!(e.read_to_string(&mut buf).unwrap(), 0);
    assert_eq!(buf, "");
    let mut buf = "hello".to_owned();
    assert_eq!(e.read_to_string(&mut buf).unwrap(), 0);
    assert_eq!(buf, "hello");
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=empty_seeks | COMPLEXITY=3 | LINES=20

```rust
#[test]
fn empty_seeks() {
    let mut e = empty();
    assert!(matches!(e.seek(SeekFrom::Start(0)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::Start(1)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::Start(u64::MAX)), Ok(0)));

    assert!(matches!(e.seek(SeekFrom::End(i64::MIN)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::End(-1)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::End(0)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::End(1)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::End(i64::MAX)), Ok(0)));

    assert!(matches!(e.seek(SeekFrom::Current(i64::MIN)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::Current(-1)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::Current(0)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::Current(1)), Ok(0)));
    assert!(matches!(e.seek(SeekFrom::Current(i64::MAX)), Ok(0)));
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=empty_sinks | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn empty_sinks() {
    test_sinking(empty());
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=repeat_repeats | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn repeat_repeats() {
    let mut r = repeat(4);
    let mut b = [0; 1024];
    assert_eq!(r.read(&mut b).unwrap(), 1024);
    assert!(b.iter().all(|b| *b == 4));
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=take_some_bytes | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn take_some_bytes() {
    assert_eq!(repeat(4).take(100).bytes().count(), 100);
    assert_eq!(repeat(4).take(100).bytes().next().unwrap().unwrap(), 4);
    assert_eq!(repeat(1).take(10).chain(repeat(2).take(10)).bytes().count(), 20);
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=const_utils | COMPLEXITY=2 | LINES=7

```rust
#[allow(dead_code)]
fn const_utils() {
    const _: Empty = empty();
    const _: Repeat = repeat(b'c');
    const _: Sink = sink();
}
```

---
*Generated by AST tracing system*
