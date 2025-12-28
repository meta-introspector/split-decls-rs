macro_rules! test_all_items {
    () => {
        # [test] fn test_all_items () { check (r#"
macro_rules! m { ($($i:item)*) => ($($i )*) }
m! {
    extern crate a;
    mod b;
    mod c {}
    use d;
    const E: i32 = 0;
    static F: i32 = 0;
    impl G {}
    struct H;
    enum I { Foo }
    trait J {}
    fn h() {}
    extern {}
    type T = u8;
}
"# , expect ! [[r#"
macro_rules! m { ($($i:item)*) => ($($i )*) }
extern crate a;
mod b;
mod c {}
use d;
const E: i32 = 0;
static F: i32 = 0;
impl G {}
struct H;
enum I {
    Foo
}
trait J {}
fn h() {}
extern {}
type T = u8;
"#]] ,) ; }
    };
}

test_all_items!()