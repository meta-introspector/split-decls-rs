// Generated macro for tests (module)
macro_rules! Depcrate_monikertests {
() => {
// Module: crate::moniker
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { MonikerResult , fixture } ; use super :: MonikerKind ; # [allow (dead_code)] # [track_caller] fn no_moniker (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let (analysis , position) = fixture :: position (ra_fixture) ; if let Some (x) = analysis . moniker (position) . unwrap () { assert_eq ! (x . info . len () , 0 , "Moniker found but no moniker expected: {x:?}") ; } } # [track_caller] fn check_local_moniker (# [rust_analyzer :: rust_fixture] ra_fixture : & str , identifier : & str , package : & str , kind : MonikerKind ,) { let (analysis , position) = fixture :: position (ra_fixture) ; let x = analysis . moniker (position) . unwrap () . expect ("no moniker found") . info ; assert_eq ! (x . len () , 1) ; match x . into_iter () . next () . unwrap () { MonikerResult :: Local { enclosing_moniker : Some (x) } => { assert_eq ! (identifier , x . identifier . to_string ()) ; assert_eq ! (package , format ! ("{:?}" , x . package_information)) ; assert_eq ! (kind , x . kind) ; } MonikerResult :: Local { enclosing_moniker : None } => { panic ! ("Unexpected local with no enclosing moniker") ; } MonikerResult :: Moniker (_) => { panic ! ("Unexpected non-local moniker") ; } } } # [track_caller] fn check_moniker (# [rust_analyzer :: rust_fixture] ra_fixture : & str , identifier : & str , package : & str , kind : MonikerKind ,) { let (analysis , position) = fixture :: position (ra_fixture) ; let x = analysis . moniker (position) . unwrap () . expect ("no moniker found") . info ; assert_eq ! (x . len () , 1) ; match x . into_iter () . next () . unwrap () { MonikerResult :: Local { enclosing_moniker } => { panic ! ("Unexpected local enclosed in {enclosing_moniker:?}") ; } MonikerResult :: Moniker (x) => { assert_eq ! (identifier , x . identifier . to_string ()) ; assert_eq ! (package , format ! ("{:?}" , x . package_information)) ; assert_eq ! (kind , x . kind) ; } } } # [test] fn basic () { check_moniker (r#"
//- /lib.rs crate:main deps:foo
use foo::module::func;
fn main() {
    func$0();
}
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub mod module {
    pub fn func() {}
}
"# , "foo::module::func" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Import ,) ; check_moniker (r#"
//- /lib.rs crate:main deps:foo
use foo::module::func;
fn main() {
    func();
}
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub mod module {
    pub fn func$0() {}
}
"# , "foo::module::func" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Export ,) ; } # [test] fn moniker_for_trait () { check_moniker (r#"
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub mod module {
    pub trait MyTrait {
        pub fn func$0() {}
    }
}
"# , "foo::module::MyTrait::func" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Export ,) ; } # [test] fn moniker_for_trait_constant () { check_moniker (r#"
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub mod module {
    pub trait MyTrait {
        const MY_CONST$0: u8;
    }
}
"# , "foo::module::MyTrait::MY_CONST" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Export ,) ; } # [test] fn moniker_for_trait_type () { check_moniker (r#"
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub mod module {
    pub trait MyTrait {
        type MyType$0;
    }
}
"# , "foo::module::MyTrait::MyType" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Export ,) ; } # [test] fn moniker_for_trait_impl_function () { check_moniker (r#"
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub mod module {
    pub trait MyTrait {
        pub fn func() {}
    }
    struct MyStruct {}
    impl MyTrait for MyStruct {
        pub fn func$0() {}
    }
}
"# , "foo::module::impl::MyStruct::MyTrait::func" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Export ,) ; } # [test] fn moniker_for_field () { check_moniker (r#"
//- /lib.rs crate:main deps:foo
use foo::St;
fn main() {
    let x = St { a$0: 2 };
}
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub struct St {
    pub a: i32,
}
"# , "foo::St::a" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Import ,) ; } # [test] fn local () { check_local_moniker (r#"
//- /lib.rs crate:main deps:foo
use foo::module::func;
fn main() {
    func();
}
//- /foo/lib.rs crate:foo@0.1.0,https://a.b/foo.git library
pub mod module {
    pub fn func() {
        let x$0 = 2;
    }
}
"# , "foo::module::func" , r#"PackageInformation { name: "foo", repo: Some("https://a.b/foo.git"), version: Some("0.1.0") }"# , MonikerKind :: Export ,) ; } }
};
}
