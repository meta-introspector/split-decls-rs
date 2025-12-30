// Generated macro for MYSQL_THREAD_UNSAFE_INIT (static)
macro_rules! Depcrate_mysql_connection_rawMYSQL_THREAD_UNSAFE_INIT {
() => {
// Module: crate::mysql::connection::raw
// Provides: {"MYSQL_THREAD_UNSAFE_INIT"}
// Dependencies: {}
# [doc = " > In a non-multi-threaded environment, `mysql_init()` invokes"] # [doc = " > `mysql_library_init()` automatically as necessary. However,"] # [doc = " > `mysql_library_init()` is not thread-safe in a multi-threaded environment,"] # [doc = " > and thus neither is `mysql_init()`. Before calling `mysql_init()`, either"] # [doc = " > call `mysql_library_init()` prior to spawning any threads, or use a mutex"] # [doc = " > to protect the `mysql_library_init()` call. This should be done prior to"] # [doc = " > any other client library call."] # [doc = ""] # [doc = " <https://dev.mysql.com/doc/c-api/8.4/en/mysql-init.html>"] static MYSQL_THREAD_UNSAFE_INIT : Once = Once :: new () ;
};
}
