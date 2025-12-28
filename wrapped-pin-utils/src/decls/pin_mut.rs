macro_rules! pin_mut {
    () => {
        # [doc = " Pins a value on the stack."] # [doc = ""] # [doc = " Can safely pin values that are not `Unpin` by taking ownership."] # [doc = ""] # [doc = " **Note:** Since Rust 1.68, this macro is soft-deprecated in favor of"] # [doc = " [`pin!`](core::pin::pin) macro in the standard library."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use pin_utils::pin_mut;"] # [doc = " # use core::pin::Pin;"] # [doc = " # struct Foo {}"] # [doc = " let foo = Foo { /* ... */ };"] # [doc = " pin_mut!(foo);"] # [doc = " let _: Pin<&mut Foo> = foo;"] # [doc = " ```"] # [macro_export] macro_rules ! pin_mut { ($ ($ x : ident) ,* $ (,) ?) => { $ (let mut $ x = $ x ; # [allow (unused_mut)] let mut $ x = unsafe { $ crate :: __private :: Pin :: new_unchecked (& mut $ x) } ;) * } }
    };
}

pin_mut!()