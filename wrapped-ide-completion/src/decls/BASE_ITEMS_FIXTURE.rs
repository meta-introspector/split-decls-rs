macro_rules! BASE_ITEMS_FIXTURE {
    () => {
        # [doc = " Lots of basic item definitions"] const BASE_ITEMS_FIXTURE : & str = r#"
enum Enum { TupleV(u32), RecordV { field: u32 }, UnitV }
use self::Enum::TupleV;
mod module {}

trait Trait {}
static STATIC: Unit = Unit;
const CONST: Unit = Unit;
struct Record { field: u32 }
struct Tuple(u32);
struct Unit;
#[macro_export]
macro_rules! makro {}
#[rustc_builtin_macro]
pub macro Clone {}
fn function() {}
union Union { field: i32 }
"# ;
    };
}

BASE_ITEMS_FIXTURE!();