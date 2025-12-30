// Generated macro for help (function)
macro_rules! Depcratehelp {
() => {
// Module: crate
// Provides: {"help"}
// Dependencies: {}
fn help () { println ! ("
Usage: {0} <command> [<args>]

Sub-commands:
    spawn-emulator <target> <server> <tmpdir> [rootfs]   See below
    push <path>                                          Copy <path> to emulator
    run <support_lib_count> <file> [support_libs...] [args...]
                                                         Run program on emulator
    help                                                 Display help message

Spawning an emulator:

For Android <target>s, adb will push the <server>, set up TCP forwarding and run
the <server>. Otherwise qemu emulates the target using a rootfs image created in
<tmpdir> and generated from <rootfs> plus the <server> executable.
If {1} is set in the environment, this step is skipped.

Pushing a path to a running emulator:

A running emulator or adb device is connected to at the IP address and port in
the {1} environment variable or {2} if this isn't
specified. The file at <path> is sent to this target.

Executing commands on a running emulator:

First the target emulator/adb session is connected to as for pushing files. Next
the <file> and any specified support libs are pushed to the target. Finally, the
<file> is executed in the emulator, preserving the current environment.
That command's status code is returned.
" , env :: args () . next () . unwrap () , REMOTE_ADDR_ENV , DEFAULT_ADDR) ; }
};
}
