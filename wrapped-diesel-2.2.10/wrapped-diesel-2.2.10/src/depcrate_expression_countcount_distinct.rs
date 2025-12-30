// Generated macro for count_distinct (function)
macro_rules! Depcrate_expression_countcount_distinct {
() => {
// Module: crate::expression::count
// Provides: {"count_distinct"}
// Dependencies: {}
# [doc = " Creates a SQL `COUNT(DISTINCT ...)` expression"] # [doc = ""] # [doc = " As with most bare functions, this is not exported by default. You can import"] # [doc = " it specifically as `diesel::dsl::count_distinct`, or glob import"] # [doc = " `diesel::dsl::*`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[macro_use] extern crate diesel;"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::*;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::posts::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let unique_user_count = posts.select(count_distinct(user_id)).first(connection);"] # [doc = " assert_eq!(Ok(2), unique_user_count);"] # [doc = " # }"] # [doc = " ```"] pub fn count_distinct < T , E > (expr : E) -> CountDistinct < T , E :: Expression > where T : SqlType + SingleValue , E : AsExpression < T > , { CountDistinct { expr : expr . as_expression () , _marker : PhantomData , } }
};
}
