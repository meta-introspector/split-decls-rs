macro_rules! test_expand_to_item_list {
    () => {
        # [test] fn test_expand_to_item_list () { check (r#"
macro_rules! structs {
    ($($i:ident),*) => { $(struct $i { field: u32 } )* }
}

// +tree
structs!(Foo, Bar);
            "# , expect ! [[r#"
macro_rules! structs {
    ($($i:ident),*) => { $(struct $i { field: u32 } )* }
}

struct Foo {
    field: u32
}
struct Bar {
    field: u32
}
// MACRO_ITEMS@0..40
//   STRUCT@0..20
//     STRUCT_KW@0..6 "struct"
//     NAME@6..9
//       IDENT@6..9 "Foo"
//     RECORD_FIELD_LIST@9..20
//       L_CURLY@9..10 "{"
//       RECORD_FIELD@10..19
//         NAME@10..15
//           IDENT@10..15 "field"
//         COLON@15..16 ":"
//         PATH_TYPE@16..19
//           PATH@16..19
//             PATH_SEGMENT@16..19
//               NAME_REF@16..19
//                 IDENT@16..19 "u32"
//       R_CURLY@19..20 "}"
//   STRUCT@20..40
//     STRUCT_KW@20..26 "struct"
//     NAME@26..29
//       IDENT@26..29 "Bar"
//     RECORD_FIELD_LIST@29..40
//       L_CURLY@29..30 "{"
//       RECORD_FIELD@30..39
//         NAME@30..35
//           IDENT@30..35 "field"
//         COLON@35..36 ":"
//         PATH_TYPE@36..39
//           PATH@36..39
//             PATH_SEGMENT@36..39
//               NAME_REF@36..39
//                 IDENT@36..39 "u32"
//       R_CURLY@39..40 "}"

            "#]] ,) ; }
    };
}

test_expand_to_item_list!();