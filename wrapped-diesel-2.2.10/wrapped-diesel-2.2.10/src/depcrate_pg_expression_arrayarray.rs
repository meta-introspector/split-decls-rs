// Generated macro for array (function)
macro_rules! Depcrate_pg_expression_arrayarray {
() => {
// Module: crate::pg::expression::array
// Provides: {"array"}
// Dependencies: {}
# [doc = " Creates an `ARRAY[...]` expression."] # [doc = ""] # [doc = " The argument should be a tuple of expressions which can be represented by the"] # [doc = " same SQL type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     run_test().unwrap();"] # [doc = " # }"] # [doc = " #"] # [doc = " # fn run_test() -> QueryResult<()> {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     use diesel::dsl::array;"] # [doc = " #     use diesel::sql_types::Integer;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let ints = diesel::select(array::<Integer, _>((1, 2)))"] # [doc = "     .get_result::<Vec<i32>>(connection)?;"] # [doc = " assert_eq!(vec![1, 2], ints);"] # [doc = ""] # [doc = " let ids = users.select(array((id, id * 2)))"] # [doc = "     .get_results::<Vec<i32>>(connection)?;"] # [doc = " let expected = vec!["] # [doc = "     vec![1, 2],"] # [doc = "     vec![2, 4],"] # [doc = " ];"] # [doc = " assert_eq!(expected, ids);"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " ```"] # [cfg (feature = "postgres_backend")] pub fn array < ST , T > (elements : T) -> ArrayLiteral < T :: Expression , ST > where T : AsExpressionList < ST > , { ArrayLiteral { elements : elements . as_expression_list () , _marker : PhantomData , } }
};
}
