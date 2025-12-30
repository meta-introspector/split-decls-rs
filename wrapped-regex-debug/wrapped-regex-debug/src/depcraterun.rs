// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run (args : & Args) -> Result < () > { if args . cmd_ast { cmd_ast (args) } else if args . cmd_prefixes { cmd_literals (args) } else if args . cmd_suffixes { cmd_literals (args) } else if args . cmd_anchors { cmd_anchors (args) } else if args . cmd_captures { cmd_captures (args) } else if args . cmd_compile { cmd_compile (args) } else { unreachable ! () } }
};
}
