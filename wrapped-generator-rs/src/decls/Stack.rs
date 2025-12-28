macro_rules! deps {
    () => {
        StackBox!();
        SysStack!();
    };
}

macro_rules! Stack {
    () => {
        deps!();
        # [doc = " generator stack"] # [doc = " this struct will not dealloc the memory"] # [doc = " instead StackBox<> would track it's usage and dealloc it"] pub struct Stack { buf : SysStack , }
    };
}

Stack!();