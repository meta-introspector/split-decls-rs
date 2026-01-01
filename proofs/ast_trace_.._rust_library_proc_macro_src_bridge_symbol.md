# AST Trace: ../rust/library/proc_macro/src/bridge/symbol.rs

Generated 12 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=STRUCT | NAME=Symbol(NonZero | COMPLEXITY=9 | LINES=22

```rust
//! Client-side interner used for symbols.
//!
//! This is roughly based on the symbol interner from `rustc_span` and the
//! DroplessArena from `rustc_arena`. It is unfortunately a complete
//! copy/re-implementation rather than a dependency as it is difficult to depend
//! on crates from within `proc_macro`, due to it being built at the same time
//! as `std`.
//!
//! If at some point in the future it becomes easier to add dependencies to
//! proc_macro, this module should probably be removed or simplified.

use std::cell::RefCell;
use std::num::NonZero;
use std::str;

use super::*;

/// Handle for a symbol string stored within the Interner.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(NonZero<u32>);

impl !Send for Symbol {}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1

```rust
impl !Sync for Symbol {}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=is_valid_ascii_ident | COMPLEXITY=42 | LINES=63

```rust
impl Symbol {
    /// Intern a new `Symbol`
    pub(crate) fn new(string: &str) -> Self {
        INTERNER.with_borrow_mut(|i| i.intern(string))
    }

    /// Creates a new `Symbol` for an identifier.
    ///
    /// Validates and normalizes before converting it to a symbol.
    pub(crate) fn new_ident(string: &str, is_raw: bool) -> Self {
        // Fast-path: check if this is a valid ASCII identifier
        if Self::is_valid_ascii_ident(string.as_bytes()) || string == "$crate" {
            if is_raw && !Self::can_be_raw(string) {
                panic!("`{}` cannot be a raw identifier", string);
            }
            return Self::new(string);
        }

        // Slow-path: If the string is already ASCII we're done, otherwise ask
        // our server to do this for us over RPC.
        // We don't need to check for identifiers which can't be raw here,
        // because all of them are ASCII.
        if string.is_ascii() {
            Err(())
        } else {
            client::Symbol::normalize_and_validate_ident(string)
        }
        .unwrap_or_else(|_| panic!("`{:?}` is not a valid identifier", string))
    }

    /// Run a callback with the symbol's string value.
    pub(crate) fn with<R>(self, f: impl FnOnce(&str) -> R) -> R {
        INTERNER.with_borrow(|i| f(i.get(self)))
    }

    /// Clear out the thread-local symbol interner, making all previously
    /// created symbols invalid such that `with` will panic when called on them.
    pub(crate) fn invalidate_all() {
        INTERNER.with_borrow_mut(|i| i.clear());
    }

    /// Checks if the ident is a valid ASCII identifier.
    ///
    /// This is a short-circuit which is cheap to implement within the
    /// proc-macro client to avoid RPC when creating simple idents, but may
    /// return `false` for a valid identifier if it contains non-ASCII
    /// characters.
    fn is_valid_ascii_ident(bytes: &[u8]) -> bool {
        matches!(bytes.first(), Some(b'_' | b'a'..=b'z' | b'A'..=b'Z'))
            && bytes[1..]
                .iter()
                .all(|b| matches!(b, b'_' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9'))
    }

    // Mimics the behavior of `Symbol::can_be_raw` from `rustc_span`
    fn can_be_raw(string: &str) -> bool {
        match string {
            "_" | "super" | "self" | "Self" | "crate" | "$crate" => false,
            _ => true,
        }
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with(|s| fmt::Debug::fmt(s, f))
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with(|s| fmt::Display::fmt(s, f))
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S> Encode<S> for Symbol {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self.with(|sym| sym.encode(w, s))
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=8

```rust
impl<S: server::Server> DecodeMut<'_, '_, server::HandleStore<server::MarkedTypes<S>>>
    for Marked<S::Symbol, Symbol>
{
    fn decode(r: &mut Reader<'_>, s: &mut server::HandleStore<server::MarkedTypes<S>>) -> Self {
        Mark::mark(S::intern_symbol(<&str>::decode(r, s)))
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=8

```rust
impl<S: server::Server> Encode<server::HandleStore<server::MarkedTypes<S>>>
    for Marked<S::Symbol, Symbol>
{
    fn encode(self, w: &mut Writer, s: &mut server::HandleStore<server::MarkedTypes<S>>) {
        S::with_symbol_string(&self.unmark(), |sym| sym.encode(w, s))
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<S> DecodeMut<'_, '_, S> for Symbol {
    fn decode(r: &mut Reader<'_>, s: &mut S) -> Self {
        Symbol::new(<&str>::decode(r, s))
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=10

```rust
thread_local! {
    static INTERNER: RefCell<Interner> = RefCell::new(Interner {
        arena: arena::Arena::new(),
        names: fxhash::FxHashMap::default(),
        strings: Vec::new(),
        // Start with a base of 1 to make sure that `NonZero<u32>` works.
        sym_base: NonZero::new(1).unwrap(),
    });
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=Interner | COMPLEXITY=5 | LINES=14

```rust
/// Basic interner for a `Symbol`, inspired by the one in `rustc_span`.
struct Interner {
    arena: arena::Arena,
    // SAFETY: These `'static` lifetimes are actually references to data owned
    // by the Arena. This is safe, as we never return them as static references
    // from `Interner`.
    names: fxhash::FxHashMap<&'static str, Symbol>,
    strings: Vec<&'static str>,
    // The offset to apply to symbol names stored in the interner. This is used
    // to ensure that symbol names are not re-used after the interner is
    // cleared.
    sym_base: NonZero<u32>,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=intern | COMPLEXITY=22 | LINES=49

```rust
impl Interner {
    fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        let name = Symbol(
            self.sym_base
                .checked_add(self.strings.len() as u32)
                .expect("`proc_macro` symbol name overflow"),
        );

        let string: &str = self.arena.alloc_str(string);

        // SAFETY: we can extend the arena allocation to `'static` because we
        // only access these while the arena is still alive.
        let string: &'static str = unsafe { &*(string as *const str) };
        self.strings.push(string);
        self.names.insert(string, name);
        name
    }

    /// Reads a symbol's value from the store while it is held.
    fn get(&self, symbol: Symbol) -> &str {
        // NOTE: Subtract out the offset which was added to make the symbol
        // nonzero and prevent symbol name re-use.
        let name = symbol
            .0
            .get()
            .checked_sub(self.sym_base.get())
            .expect("use-after-free of `proc_macro` symbol");
        self.strings[name as usize]
    }

    /// Clear all symbols from the store, invalidating them such that `get` will
    /// panic if they are accessed in the future.
    fn clear(&mut self) {
        // NOTE: Be careful not to panic here, as we may be called on the client
        // when a `catch_unwind` isn't installed.
        self.sym_base = self.sym_base.saturating_add(self.strings.len() as u32);
        self.names.clear();
        self.strings.clear();

        // SAFETY: This is cleared after the names and strings tables are
        // cleared out, so no references into the arena should remain.
        self.arena = arena::Arena::new();
    }
}
```

---
*Generated by AST tracing system*
