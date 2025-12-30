// Generated macro for CLIENT_USAGE (const)
macro_rules! Depcrate_argsCLIENT_USAGE {
() => {
// Module: crate::args
// Provides: {"CLIENT_USAGE"}
// Dependencies: {}
pub const CLIENT_USAGE : & str = "Usage:
  quiche-client [options] URL...
  quiche-client -h | --help

Options:
  --method METHOD          Use the given HTTP request method [default: GET].
  --body FILE              Send the given file as request body.
  --max-data BYTES         Connection-wide flow control limit [default: 10000000].
  --max-window BYTES       Connection-wide max receiver window [default: 25165824].
  --max-stream-data BYTES  Per-stream flow control limit [default: 1000000].
  --max-stream-window BYTES   Per-stream max receiver window [default: 16777216].
  --max-streams-bidi STREAMS  Number of allowed concurrent streams [default: 100].
  --max-streams-uni STREAMS   Number of allowed concurrent streams [default: 100].
  --idle-timeout TIMEOUT   Idle timeout in milliseconds [default: 30000].
  --wire-version VERSION   The version number to send to the server [default: babababa].
  --http-version VERSION   HTTP version to use [default: all].
  --early-data             Enable sending early data.
  --dgram-proto PROTO      DATAGRAM application protocol to use [default: none].
  --dgram-count COUNT      Number of DATAGRAMs to send [default: 0].
  --dgram-data DATA        Data to send for certain types of DATAGRAM application protocol [default: quack].
  --dump-packets PATH      Dump the incoming packets as files in the given directory.
  --dump-responses PATH    Dump response payload as files in the given directory.
  --dump-json              Dump response headers and payload to stdout in JSON format.
  --max-json-payload BYTES  Per-response payload limit when dumping JSON [default: 10000].
  --connect-to ADDRESS     Override the server's address.
  --no-verify              Don't verify server's certificate.
  --trust-origin-ca-pem <file>  Path to the pem file of the origin's CA, if not publicly trusted.
  --no-grease              Don't send GREASE.
  --cc-algorithm NAME      Specify which congestion control algorithm to use [default: cubic].
  --disable-hystart        Disable HyStart++.
  --max-active-cids NUM    The maximum number of active Connection IDs we can support [default: 2].
  --enable-active-migration   Enable active connection migration.
  --perform-migration      Perform connection migration on another source port.
  -H --header HEADER ...   Add a request header.
  -n --requests REQUESTS   Send the given number of identical requests [default: 1].
  --send-priority-update   Send HTTP/3 priority updates if the query string params 'u' or 'i' are present in URLs
  --max-field-section-size BYTES    Max size of uncompressed field section. Default is unlimited.
  --qpack-max-table-capacity BYTES  Max capacity of dynamic QPACK decoding.. Any value other that 0 is currently unsupported.
  --qpack-blocked-streams STREAMS   Limit of blocked streams while decoding. Any value other that 0 is currently unsupported.
  --session-file PATH      File used to cache a TLS session for resumption.
  --source-port PORT       Source port to use when connecting to the server [default: 0].
  --initial-rtt MILLIS     The initial RTT in milliseconds [default: 333].
  --initial-cwnd-packets PACKETS   The initial congestion window size in terms of packet count [default: 10].
  -h --help                Show this screen.
" ;
};
}
