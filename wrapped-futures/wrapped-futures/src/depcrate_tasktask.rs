// Generated macro for Task (struct)
macro_rules! Depcrate_taskTask {
() => {
// Module: crate::task
// Provides: {"Task"}
// Dependencies: {}
# [doc = " A structure representing one \"task\", or thread of execution throughout the"] # [doc = " lifetime of a set of futures."] # [doc = ""] # [doc = " It's intended that futures are composed together to form a large \"task\" of"] # [doc = " futures which is driven as a whole throughout its lifetime. This task is"] # [doc = " persistent for the entire lifetime of the future until its completion,"] # [doc = " carrying any local data and such."] # [doc = ""] # [doc = " Currently tasks serve two primary purposes:"] # [doc = ""] # [doc = " * They're used to drive futures to completion, e.g. executors (more to be"] # [doc = "   changed here soon)."] # [doc = " * They store task local data. That is, any task can contain any number of"] # [doc = "   pieces of arbitrary data which can be accessed at a later date. The data"] # [doc = "   is owned and carried in the task itself, and `TaskData` handles are used"] # [doc = "   to access the internals."] # [doc = ""] # [doc = " This structure is likely to expand more customizable functionality over"] # [doc = " time! That is, it's not quite done yet..."] pub struct Task { handle : TaskHandle , _marker : marker :: PhantomData < Cell < () > > , }
};
}
