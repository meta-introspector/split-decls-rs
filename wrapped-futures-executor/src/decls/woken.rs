macro_rules! woken {
    () => {
        # [doc = " Check for a wakeup, but don't consume it."] fn woken () -> bool { CURRENT_THREAD_NOTIFY . with (| thread_notify | thread_notify . unparked . load (Ordering :: Acquire)) }
    };
}

woken!();