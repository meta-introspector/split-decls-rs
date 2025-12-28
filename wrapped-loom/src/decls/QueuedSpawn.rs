macro_rules! QueuedSpawn {
    () => {
        struct QueuedSpawn { f : Box < dyn FnOnce () > , stack_size : Option < usize > , }
    };
}

QueuedSpawn!()