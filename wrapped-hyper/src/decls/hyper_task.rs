macro_rules! deps {
    () => {
        UserDataPointer!();
        BoxFuture!();
        BoxAny!();
    };
}

macro_rules! hyper_task {
    () => {
        deps!();
        # [doc = " An async task."] # [doc = ""] # [doc = " A task represents a chunk of work that will eventually yield exactly one"] # [doc = " `hyper_task_value`. Tasks are pushed onto an executor, and that executor is"] # [doc = " responsible for calling the necessary private functions on the task to make"] # [doc = " progress. In most cases those private functions will eventually cause read"] # [doc = " or write callbacks on a `hyper_io` object to be called."] # [doc = ""] # [doc = " Tasks are created by various functions:"] # [doc = ""] # [doc = " - hyper_clientconn_handshake: Creates an HTTP client handshake task."] # [doc = " - hyper_clientconn_send:      Creates a task to send a request on the client connection."] # [doc = " - hyper_body_data:            Creates a task that will poll a response body for the next buffer of data."] # [doc = " - hyper_body_foreach:         Creates a task to execute the callback with each body chunk received."] # [doc = ""] # [doc = " Tasks then have a userdata associated with them using `hyper_task_set_userdata`. This"] # [doc = " is important, for instance, to associate a request id with a given request. When multiple"] # [doc = " tasks are running on the same executor, this allows distinguishing tasks for different"] # [doc = " requests."] # [doc = ""] # [doc = " Tasks are then pushed onto an executor, and eventually yielded from hyper_executor_poll:"] # [doc = ""] # [doc = " - hyper_executor_push:        Push a task onto the executor."] # [doc = " - hyper_executor_poll:        Polls the executor, trying to make progress on any tasks that have notified that they are ready again."] # [doc = ""] # [doc = " Once a task is yielded from poll, retrieve its userdata, check its type,"] # [doc = " and extract its value. This will require a case from void* to the appropriate type."] # [doc = ""] # [doc = " Methods on hyper_task:"] # [doc = ""] # [doc = " - hyper_task_type:            Query the return type of this task."] # [doc = " - hyper_task_value:           Takes the output value of this task."] # [doc = " - hyper_task_set_userdata:    Set a user data pointer to be associated with this task."] # [doc = " - hyper_task_userdata:        Retrieve the userdata that has been set via hyper_task_set_userdata."] # [doc = " - hyper_task_free:            Free a task."] pub struct hyper_task { future : BoxFuture < BoxAny > , output : Option < BoxAny > , userdata : UserDataPointer , }
    };
}

hyper_task!();