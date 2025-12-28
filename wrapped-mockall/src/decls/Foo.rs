macro_rules! Foo {
    () => {
        # [doc = " Mock of a basic trait with several kinds of method."] # [doc = ""] # [doc = " It is mocked by the [`MockFoo`](struct.MockFoo.html) struct."] # [automock] pub trait Foo { # [doc = " A method with a `'static` return type"] fn foo (& self , x : i32 , y : i16) -> i32 ; # [doc = " A method returning a reference"] fn bar (& self , x : i32) -> & i32 ; # [doc = " A method returning a mutable reference"] fn baz (& mut self , x : i32) -> & mut i32 ; # [doc = " A method returning a `'static` reference"] fn bean (& self) -> & 'static i32 ; # [doc = " A static method"] fn bang (x : i32) -> i32 ; }
    };
}

Foo!()