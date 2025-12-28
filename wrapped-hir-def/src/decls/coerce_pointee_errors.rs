macro_rules! deps {
    () => {
        Enum!();
    };
}

macro_rules! coerce_pointee_errors {
    () => {
        deps!();
        # [test] fn coerce_pointee_errors () { check_errors (r#"
//- minicore: coerce_pointee

use core::marker::CoercePointee;

#[derive(CoercePointee)]
enum Enum {}

#[derive(CoercePointee)]
struct Struct1;

#[derive(CoercePointee)]
struct Struct2();

#[derive(CoercePointee)]
struct Struct3 {}

#[derive(CoercePointee)]
struct Struct4<T: ?Sized>(T);

#[derive(CoercePointee)]
#[repr(transparent)]
struct Struct5(i32);

#[derive(CoercePointee)]
#[repr(transparent)]
struct Struct6<#[pointee] T: ?Sized, #[pointee] U: ?Sized>(T, U);

#[derive(CoercePointee)]
#[repr(transparent)]
struct Struct7<T: ?Sized, U: ?Sized>(T, U);

#[derive(CoercePointee)]
#[repr(transparent)]
struct Struct8<#[pointee] T, U: ?Sized>(T);

#[derive(CoercePointee)]
#[repr(transparent)]
struct Struct9<T>(T);

#[derive(CoercePointee)]
#[repr(transparent)]
struct Struct9<#[pointee] T, U>(T) where T: ?Sized;
"# , expect ! [[r#"
            35..72: `CoercePointee` can only be derived on `struct`s
            74..114: `CoercePointee` can only be derived on `struct`s with at least one field
            116..158: `CoercePointee` can only be derived on `struct`s with at least one field
            160..202: `CoercePointee` can only be derived on `struct`s with at least one field
            204..258: `CoercePointee` can only be derived on `struct`s with `#[repr(transparent)]`
            260..326: `CoercePointee` can only be derived on `struct`s that are generic over at least one type
            328..439: only one type parameter can be marked as `#[pointee]` when deriving `CoercePointee` traits
            441..530: exactly one generic type parameter must be marked as `#[pointee]` to derive `CoercePointee` traits
            532..621: `derive(CoercePointee)` requires `T` to be marked `?Sized`
            623..690: `derive(CoercePointee)` requires `T` to be marked `?Sized`"#]] ,) ; }
    };
}

coerce_pointee_errors!()