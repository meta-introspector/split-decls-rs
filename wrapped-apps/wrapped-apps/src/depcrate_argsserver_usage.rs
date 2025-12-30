// Generated macro for SERVER_USAGE (const)
macro_rules! Depcrate_argsSERVER_USAGE {
() => {
// Module: crate::args
// Provides: {"SERVER_USAGE"}
// Dependencies: {}
pub const SERVER_USAGE : & str = "Usage:
  quiche-server [options]
  quiche-server -h | --help

Options:
  --listen <addr>             Listen on the given IP:port [default: 127.0.0.1:4433]
  --cert <file>               TLS certificate path [default: src/bin/cert.crt]
  --key <file>                TLS certificate key path [default: src/bin/cert.key]
  --root <dir>                Root directory [default: src/bin/root/]
  --index <name>              The file that will be used as index [default: index.html].
  --name <str>                Name of the server [default: quic.tech]
  --max-data BYTES            Connection-wide flow control limit [default: 10000000].
  --max-window BYTES          Connection-wide max receiver window [default: 25165824].
  --max-stream-data BYTES     Per-stream flow control limit [default: 1000000].
  --max-stream-window BYTES   Per-stream max receiver window [default: 16777216].
  --max-streams-bidi STREAMS  Number of allowed concurrent streams [default: 100].
  --max-streams-uni STREAMS   Number of allowed concurrent streams [default: 100].
  --idle-timeout TIMEOUT      Idle timeout in milliseconds [default: 30000].
  --dump-packets PATH         Dump the incoming packets as files in the given directory.
  --early-data                Enable receiving early data.
  --no-retry                  Disable stateless retry.
  --no-grease                 Don't send GREASE.
  --http-version VERSION      HTTP version to use [default: all].
  --dgram-proto PROTO         DATAGRAM application protocol to use [default: none].
  --dgram-count COUNT         Number of DATAGRAMs to send [default: 0].
  --dgram-data DATA           Data to send for certain types of DATAGRAM application protocol [default: brrr].
  --cc-algorithm NAME         Specify which congestion control algorithm to use [default: cubic].
  --disable-hystart           Disable HyStart++.
  --max-active-cids NUM       The maximum number of active Connection IDs we can support [default: 2].
  --enable-active-migration   Enable active connection migration.
  --max-field-section-size BYTES    Max size of uncompressed HTTP/3 field section. Default is unlimited.
  --qpack-max-table-capacity BYTES  Max capacity of QPACK dynamic table decoding. Any value other that 0 is currently unsupported.
  --qpack-blocked-streams STREAMS   Limit of streams that can be blocked while decoding. Any value other that 0 is currently unsupported.
  --disable-gso               Disable GSO (linux only).
  --disable-pacing            Disable pacing (linux only).
  --initial-rtt MILLIS     The initial RTT in milliseconds [default: 333].
  --initial-cwnd-packets PACKETS      The initial congestion window size in terms of packet count [default: 10].
  -h --help                   Show this screen.
" ;
};
}
