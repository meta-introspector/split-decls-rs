use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Import a mock type in test mode, or a real type otherwise.
///
/// In a regular build, this macro is a no-op.  But when `#[cfg(test)]`, it
/// substitutes "MockXXX" for any imported "XXX".  This makes it easy to to use
/// mock objects, especially structs, in your unit tests.
///
/// This macro uses the same naming convention as
/// [`mockall`](https://docs.rs/mockall/latest/mockall), but doesn't strictly
/// require it.  It can work with hand-built Mock objects, or any other crate
/// using the same naming convention.
///
/// This is the most common way to use `#[double]`.  In order to replace a type,
/// it must come from a separate module.  So to mock type `Foo`, place it into a
/// submodule, along with its mock counterpart.  Then simply import `Foo` with
/// `#[double]` like this:
/// ```no_run
/// # use mockall_double::double;
/// mod foo {
///     pub(super) struct Foo {
///         // ...
///     }
///     #[cfg(test)]
///     pub(super) struct MockFoo {
///         // ...
///     }
/// }
/// #[double]
/// use foo::Foo;
/// ```
/// That will expand to the following:
/// ```no_run
/// # use mockall_double::double;
/// # mod foo { pub struct Foo {} }
/// #[cfg(not(test))]
/// use foo::Foo;
/// #[cfg(test)]
/// use foo::MockFoo as Foo;
/// ```
/// `#[double]` can handle deeply nested paths,
/// ```no_run
/// # use mockall_double::double;
/// # mod foo { pub mod bar { pub struct Baz{} } }
/// #[double]
/// use foo::bar::Baz;
/// ```
/// grouped imports,
/// ```no_run
/// # mod foo { pub mod bar { pub struct Baz{} pub struct Bean {} } }
/// # use mockall_double::double;
/// #[double]
/// use foo::bar::{
///     Baz,
///     Bean
/// };
/// ```
/// type aliases,
/// ```no_run
/// # mod bar { pub struct Baz {} }
/// # use mockall_double::double;
/// #[double]
/// type Foo = bar::Baz;
/// ```
/// and renamed imports, too.  With renamed imports, it isn't even necessary to
/// declare a submodule.
/// ```no_run
/// # use mockall_double::double;
/// # struct Foo {}
/// #[double]
/// use Foo as Bar;
/// ```
/// Finally, `#[double]` can also import entire mocked modules, not just
/// structures.  In this case the naming convention is different.  It will
/// replace "xxx" with "mock_xxx".  For example:
/// ```no_run
/// # use mockall_double::double;
/// mod foo {
///     pub mod inner {
///         // ...
///     }
///     pub mod mock_inner {
///         // ...
///     }
/// }
/// #[double]
/// use foo::inner;
/// ```
/// will expand to:
/// ```no_run
/// # use mockall_double::double;
/// # mod foo { pub mod inner { } pub mod mock_inner { } }
/// #[cfg(not(test))]
/// use foo::inner;
/// #[cfg(test)]
/// use foo::mock_inner as inner;
/// ```
///
#[proc_macro_attribute]
pub fn double(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    do_double(attrs.into(), input.into()).into()
}
